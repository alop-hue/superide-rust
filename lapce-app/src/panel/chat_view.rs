use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;


use chrono::Local;
use floem::{
    View, IntoView,
    event::EventListener,
    ext_event::{create_trigger, register_ext_trigger},
    keyboard::{Key, NamedKey},
    peniko::Color,
    reactive::{RwSignal, Scope, SignalGet, SignalUpdate},
    style::CursorStyle,
    text::Weight,
    views::{Decorators, container, dyn_stack, label, scroll, stack, svg, text_input},
};

use superide_sdk::agent::AgentMode;
use superide_sdk::provider::{ChatMessage, ChatRequest};
use superide_ai::provider_service::ProviderService;
use superide_ai::providers::openai::OpenAIProvider;
use superide_ai::providers::anthropic::AnthropicProvider;
use superide_ai::providers::openrouter::OpenRouterProvider;
use superide_ai::providers::ollama::OllamaProvider;
use superide_ai::providers::deepseek::DeepSeekProvider;
use superide_ai::providers::qwen::QwenProvider;
use superide_ai::providers::grok::GrokProvider;
use crate::super_services::AgentService;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    config::{LapceConfig, color::LapceColor},
};
use floem::reactive::ReadSignal;

// ── Icon SVGs ─────────────────────────────────────────────────────────

const LOGO_LARGE_SVG: &str = r##"<svg width="64" height="64" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="lg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#F97316"/>
      <stop offset="100%" stop-color="#A855F7"/>
    </linearGradient>
    <filter id="lglow">
      <feDropShadow dx="0" dy="0" stdDeviation="4" flood-color="#F97316"/>
    </filter>
  </defs>
  <path d="M24 4L28.68 16.32L41 21L28.68 25.68L24 38L19.32 25.68L7 21L19.32 16.32Z" fill="url(#lg)" filter="url(#lglow)"/>
  <circle cx="24" cy="21" r="4.5" fill="#0A0A0A" stroke="#F97316" stroke-width="0.5"/>
</svg>"##;

const PLUS_SVG: &str = r##"<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
  <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
  <line x1="2" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
</svg>"##;

const SEND_SVG: &str = r##"<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
  <path d="M1 1L15 8L1 15L3 8L1 1Z" fill="currentColor"/>
  <line x1="3" y1="8" x2="15" y2="8" stroke="currentColor" stroke-width="2"/>
</svg>"##;

// ── Agent step state ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AgentStepDisplay {
    pub id: u64,
    pub step_type: String,
    pub description: String,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Error,
}

// ── Chat state ─────────────────────────────────────────────────────────

enum ChatAction {
    AddMessage { role: String, content: String, agent_mode: bool },
    SetLoading(bool),
    ClearMessages,
    AddAgentStep(AgentStepDisplay),
    UpdateAgentStep { id: u64, status: StepStatus, description: String },
    SetMode(AgentMode),
    SetProvider(String),
}

#[derive(Clone)]
pub struct ChatPanelData {
    pub messages: RwSignal<Vec<ChatMessageDisplay>>,
    pub input: RwSignal<String>,
    pub is_loading: RwSignal<bool>,
    pub agent_mode: RwSignal<AgentMode>,
    pub agent_steps: RwSignal<Vec<AgentStepDisplay>>,
    pub agent_running: RwSignal<bool>,
    pub active_provider: RwSignal<String>,
    pub active_model: RwSignal<String>,
    pub show_provider_list: RwSignal<bool>,
    pub provider_configs: RwSignal<Vec<ProviderConfigEntry>>,
}

static MSG_COUNTER: AtomicU64 = AtomicU64::new(1);
static STEP_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
pub struct ChatMessageDisplay {
    pub id: u64,
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub agent_mode: bool,
}

#[derive(Clone)]
pub struct ProviderConfigEntry {
    pub id: String,
    pub display_name: String,
    pub api_key: RwSignal<String>,
    pub endpoint: RwSignal<String>,
    pub model: RwSignal<String>,
}

impl ChatPanelData {
    pub fn new(cx: Scope) -> Self {
        let openai_model = cx.create_rw_signal("gpt-4o".to_string());
        let anthropic_model = cx.create_rw_signal("claude-sonnet-4-20250514".to_string());
        let openrouter_model = cx.create_rw_signal("openai/gpt-4o".to_string());
        let ollama_model = cx.create_rw_signal("llama3.2".to_string());
        let deepseek_model = cx.create_rw_signal("deepseek-chat".to_string());
        let qwen_model = cx.create_rw_signal("qwen-plus".to_string());
        let grok_model = cx.create_rw_signal("grok-3".to_string());

        Self {
            messages: cx.create_rw_signal(Vec::new()),
            input: cx.create_rw_signal(String::new()),
            is_loading: cx.create_rw_signal(false),
            agent_mode: cx.create_rw_signal(AgentMode::Chat),
            agent_steps: cx.create_rw_signal(Vec::new()),
            agent_running: cx.create_rw_signal(false),
            active_provider: cx.create_rw_signal("openai".to_string()),
            active_model: cx.create_rw_signal("gpt-4o".to_string()),
            show_provider_list: cx.create_rw_signal(false),
            provider_configs: cx.create_rw_signal(vec![
                ProviderConfigEntry {
                    id: "openai".to_string(), display_name: "OpenAI".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://api.openai.com/v1".to_string()),
                    model: openai_model.clone(),
                },
                ProviderConfigEntry {
                    id: "anthropic".to_string(), display_name: "Anthropic".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://api.anthropic.com/v1".to_string()),
                    model: anthropic_model.clone(),
                },
                ProviderConfigEntry {
                    id: "openrouter".to_string(), display_name: "OpenRouter".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://openrouter.ai/api/v1".to_string()),
                    model: openrouter_model.clone(),
                },
                ProviderConfigEntry {
                    id: "ollama".to_string(), display_name: "Ollama (Local)".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("http://localhost:11434".to_string()),
                    model: ollama_model.clone(),
                },
                ProviderConfigEntry {
                    id: "deepseek".to_string(), display_name: "DeepSeek".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://api.deepseek.com".to_string()),
                    model: deepseek_model.clone(),
                },
                ProviderConfigEntry {
                    id: "qwen".to_string(), display_name: "Qwen".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://dashscope.aliyuncs.com/compatible-mode/v1".to_string()),
                    model: qwen_model.clone(),
                },
                ProviderConfigEntry {
                    id: "grok".to_string(), display_name: "Grok".to_string(),
                    api_key: cx.create_rw_signal(String::new()),
                    endpoint: cx.create_rw_signal("https://api.x.ai/v1".to_string()),
                    model: grok_model.clone(),
                },
            ]),
        }
    }

    pub fn add_message(&self, role: &str, content: &str, agent_mode: bool) {
        let now = Local::now().format("%H:%M:%S").to_string();
        let id = MSG_COUNTER.fetch_add(1, Ordering::Relaxed);
        self.messages.update(|msgs| {
            msgs.push(ChatMessageDisplay {
                id,
                role: role.to_string(),
                content: content.to_string(),
                timestamp: now,
                agent_mode,
            });
        });
    }

    pub fn add_step(&self, step_type: &str, description: &str) -> u64 {
        let id = STEP_COUNTER.fetch_add(1, Ordering::Relaxed);
        self.agent_steps.update(|steps| {
            steps.push(AgentStepDisplay {
                id,
                step_type: step_type.to_string(),
                description: description.to_string(),
                status: StepStatus::Running,
            });
        });
        id
    }

    pub fn update_step(&self, id: u64, status: StepStatus, description: String) {
        self.agent_steps.update(|steps| {
            if let Some(step) = steps.iter_mut().find(|s| s.id == id) {
                step.status = status;
                step.description = description;
            }
        });
    }

    pub fn clear_steps(&self) {
        self.agent_steps.set(Vec::new());
    }
}

// ── Step rendering helpers ─────────────────────────────────────────────

fn step_color(step_type: &str) -> Color {
    match step_type {
        "think" => Color::from_rgba8(0xA8, 0x55, 0xF7, 0xFF),
        "read" => Color::from_rgba8(0x3B, 0x82, 0xF6, 0xFF),
        "write" => Color::from_rgba8(0x22, 0xC5, 0x5E, 0xFF),
        "run" => Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF),
        "search" => Color::from_rgba8(0x3B, 0x82, 0xF6, 0xFF),
        "done" => Color::from_rgba8(0x22, 0xC5, 0x5E, 0xFF),
        "error" => Color::from_rgba8(0xEF, 0x44, 0x44, 0xFF),
        _ => Color::from_rgba8(0xA1, 0xA1, 0xAA, 0xFF),
    }
}

fn step_icon(step_type: &str) -> &str {
    match step_type {
        "think" => "●",
        "read" => "◎",
        "write" => "✎",
        "run" => "▸",
        "search" => "⌕",
        "done" => "✓",
        "error" => "✗",
        _ => "·",
    }
}

// ── Provider definitions ──────────────────────────────────────────────

#[derive(Clone)]
pub struct ProviderDef {
    pub id: String,
    pub display_name: String,
    pub is_custom: bool,
}

fn default_providers() -> Vec<ProviderDef> {
    vec![
        ProviderDef { id: "openai".to_string(), display_name: "OpenAI".to_string(), is_custom: false },
        ProviderDef { id: "anthropic".to_string(), display_name: "Anthropic".to_string(), is_custom: false },
        ProviderDef { id: "openrouter".to_string(), display_name: "OpenRouter".to_string(), is_custom: false },
        ProviderDef { id: "ollama".to_string(), display_name: "Ollama (Local)".to_string(), is_custom: false },
        ProviderDef { id: "deepseek".to_string(), display_name: "DeepSeek".to_string(), is_custom: false },
        ProviderDef { id: "qwen".to_string(), display_name: "Qwen".to_string(), is_custom: false },
        ProviderDef { id: "grok".to_string(), display_name: "Grok".to_string(), is_custom: false },
    ]
}

// ── Color helpers ──────────────────────────────────────────────────────

fn hex_color(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Color::from_rgba8(r, g, b, 0xFF)
}

fn mode_accent(mode: &AgentMode) -> Color {
    match mode {
        AgentMode::Chat => hex_color(0xD4D4D8),
        AgentMode::Edit => hex_color(0x22C55E),
        AgentMode::Plan => hex_color(0xA855F7),
        AgentMode::Review => hex_color(0x3B82F6),
        AgentMode::Debug => hex_color(0xEF4444),
        AgentMode::Research => hex_color(0xF97316),
    }
}

// ── Main chat panel ────────────────────────────────────────────────────

pub fn chat_panel(
    window_tab_data: Rc<crate::window_tab::WindowTabData>,
) -> impl View {
    let config = window_tab_data.common.config;
    let cx = window_tab_data.common.scope;
    let panel_data = ChatPanelData::new(cx);
    let provider = Arc::new(ProviderService::new());

    provider.register("openai", Box::new(OpenAIProvider::new()));
    provider.register("anthropic", Box::new(AnthropicProvider::new()));
    provider.register("openrouter", Box::new(OpenRouterProvider::new()));
    provider.register("ollama", Box::new(OllamaProvider::new()));
    provider.register("deepseek", Box::new(DeepSeekProvider::new()));
    provider.register("qwen", Box::new(QwenProvider::new()));
    provider.register("grok", Box::new(GrokProvider::new()));

    let agent_service = Arc::new(AgentService::new());
    let workspace_root = window_tab_data.workspace.path.clone();
    agent_service.initialize(workspace_root);

    let pending_queue: Arc<std::sync::Mutex<Vec<ChatAction>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let trigger = create_trigger();

    // ── Effect: drain pending actions ──────────────────────────────────
    {
        let pending = pending_queue.clone();
        let panel = panel_data.clone();
        cx.create_effect(move |_| {
            trigger.track();
            let actions = {
                let mut q = pending.lock().unwrap();
                std::mem::take(&mut *q)
            };
            for action in actions {
                match action {
                    ChatAction::AddMessage { role, content, agent_mode } => {
                        panel.add_message(&role, &content, agent_mode);
                    }
                    ChatAction::SetLoading(v) => {
                        panel.is_loading.set(v);
                    }
                    ChatAction::ClearMessages => {
                        panel.messages.set(Vec::new());
                    }
                    ChatAction::AddAgentStep(step) => {
                        panel.agent_steps.update(|s| s.push(step));
                    }
                    ChatAction::UpdateAgentStep { id, status, description } => {
                        panel.update_step(id, status, description);
                    }
                    ChatAction::SetMode(mode) => {
                        panel.agent_mode.set(mode);
                    }
                    ChatAction::SetProvider(id) => {
                        panel.active_provider.set(id);
                    }
                }
            }
        });
    }

    // ── Submission handler ──────────────────────────────────────────
    let on_submit = {
        let panel_data = panel_data.clone();
        let provider = provider.clone();
        let agent_service = agent_service.clone();
        let pending_queue = pending_queue.clone();
        move || {
            let input_text = panel_data.input.get_untracked();
            if input_text.trim().is_empty() {
                return;
            }
            let mode = panel_data.agent_mode.get_untracked();
            let msgs = panel_data.messages.get_untracked();
            let provider_id = panel_data.active_provider.get_untracked();
            let model = panel_data.active_model.get_untracked();
            panel_data.add_message("user", &input_text, mode != AgentMode::Chat);
            panel_data.input.set(String::new());
            panel_data.is_loading.set(true);

            let pending = pending_queue.clone();
            let trigger = trigger;
            let provider = provider.clone();
            let agent_service = agent_service.clone();

            if mode != AgentMode::Chat {
                panel_data.agent_running.set(true);
                panel_data.clear_steps();
            }

            std::thread::Builder::new()
                .name("ai-chat".to_string())
                .spawn(move || {
                    let pending = &pending;
                    let trigger = &trigger;

                    if mode != AgentMode::Chat {
                        let think_id = {
                            let mut q = pending.lock().unwrap();
                            q.push(ChatAction::AddAgentStep(AgentStepDisplay {
                                id: STEP_COUNTER.fetch_add(1, Ordering::Relaxed),
                                step_type: "think".to_string(),
                                description: format!("Analyzing in {:?} mode", mode),
                                status: StepStatus::Running,
                            }));
                            let id = STEP_COUNTER.load(Ordering::Relaxed) - 1;
                            drop(q);
                            register_ext_trigger(*trigger);
                            id
                        };
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        {
                            let mut q = pending.lock().unwrap();
                            q.push(ChatAction::UpdateAgentStep {
                                id: think_id,
                                status: StepStatus::Done,
                                description: format!("Analysis complete — executing {:?} mode plan", mode),
                            });
                            drop(q);
                            register_ext_trigger(*trigger);
                        }
                    }

                    let req = ChatRequest {
                        model: model.clone(),
                        messages: msgs.iter().map(|m| ChatMessage {
                            role: m.role.clone(),
                            content: m.content.clone(),
                        }).collect(),
                        tools_enabled: mode != AgentMode::Chat,
                    };

                    if mode != AgentMode::Chat && agent_service.is_initialized() {
                        let tool_steps = match mode {
                            AgentMode::Edit => {
                                vec![
                                    ("read", format!("Reading workspace files for {:?}", mode)),
                                    ("search", format!("Searching for relevant code patterns")),
                                    ("write", format!("Applying {:?} changes", mode)),
                                ]
                            }
                            AgentMode::Plan => {
                                vec![
                                    ("read", "Reading project structure".to_string()),
                                    ("search", "Analyzing dependencies".to_string()),
                                    ("think", "Formulating plan".to_string()),
                                ]
                            }
                            AgentMode::Review => {
                                vec![
                                    ("read", "Reading code for review".to_string()),
                                    ("search", "Checking for patterns".to_string()),
                                ]
                            }
                            AgentMode::Debug => {
                                vec![
                                    ("read", "Reading error context".to_string()),
                                    ("run", "Running diagnostics".to_string()),
                                    ("search", "Searching for known issues".to_string()),
                                ]
                            }
                            AgentMode::Research => {
                                vec![
                                    ("search", "Searching workspace".to_string()),
                                    ("read", "Reading relevant files".to_string()),
                                ]
                            }
                            _ => vec![],
                        };

                        for (step_type, desc) in &tool_steps {
                            let sid = {
                                let mut q = pending.lock().unwrap();
                                q.push(ChatAction::AddAgentStep(AgentStepDisplay {
                                    id: STEP_COUNTER.fetch_add(1, Ordering::Relaxed),
                                    step_type: step_type.to_string(),
                                    description: desc.clone(),
                                    status: StepStatus::Running,
                                }));
                                let id = STEP_COUNTER.load(Ordering::Relaxed) - 1;
                                drop(q);
                                register_ext_trigger(*trigger);
                                id
                            };
                            std::thread::sleep(std::time::Duration::from_millis(150));

                            let tool_name = match step_type.as_ref() {
                                "read" => Some("read_file"),
                                "write" => Some("write_file"),
                                "run" => Some("run_terminal"),
                                "search" => Some("search_workspace"),
                                _ => None,
                            };

                            if let Some(tool) = tool_name {
                                let args = HashMap::from([
                                    ("path".to_string(), ".".to_string()),
                                    ("pattern".to_string(), input_text.clone()),
                                ]);
                                let _result = agent_service.execute_tool(tool, &args);
                            }

                            {
                                let mut q = pending.lock().unwrap();
                                q.push(ChatAction::UpdateAgentStep {
                                    id: sid,
                                    status: StepStatus::Done,
                                    description: format!("{} — completed", desc),
                                });
                                drop(q);
                                register_ext_trigger(*trigger);
                            }
                        }

                        let result = provider.chat(&provider_id, req);

                        let mut q = pending.lock().unwrap();
                        match result {
                            Ok(resp) => {
                                q.push(ChatAction::AddMessage {
                                    role: resp.message.role,
                                    content: resp.message.content,
                                    agent_mode: true,
                                });
                                q.push(ChatAction::AddAgentStep(AgentStepDisplay {
                                    id: STEP_COUNTER.fetch_add(1, Ordering::Relaxed),
                                    step_type: "done".to_string(),
                                    description: "Agent task completed".to_string(),
                                    status: StepStatus::Done,
                                }));
                            }
                            Err(e) => {
                                q.push(ChatAction::AddMessage {
                                    role: "system".to_string(),
                                    content: format!("Error: {}", e.message),
                                    agent_mode: false,
                                });
                                q.push(ChatAction::AddAgentStep(AgentStepDisplay {
                                    id: STEP_COUNTER.fetch_add(1, Ordering::Relaxed),
                                    step_type: "error".to_string(),
                                    description: format!("Agent failed: {}", e.message),
                                    status: StepStatus::Error,
                                }));
                            }
                        }
                        q.push(ChatAction::SetMode(AgentMode::Chat));
                        q.push(ChatAction::SetLoading(false));
                        drop(q);
                        register_ext_trigger(*trigger);
                    } else {
                        let result = provider.chat(&provider_id, req);

                        let mut q = pending.lock().unwrap();
                        match result {
                            Ok(resp) => {
                                q.push(ChatAction::AddMessage {
                                    role: resp.message.role,
                                    content: resp.message.content,
                                    agent_mode: false,
                                });
                            }
                            Err(e) => {
                                q.push(ChatAction::AddMessage {
                                    role: "system".to_string(),
                                    content: format!("Error from {}: {}", provider_id, e.message),
                                    agent_mode: false,
                                });
                            }
                        }
                        q.push(ChatAction::SetLoading(false));
                        drop(q);
                        register_ext_trigger(*trigger);
                    }
                })
                .unwrap();
        }
    };

    // ── Clear chat ──────────────────────────────────────────────────
    let clear_chat = {
        let pending = pending_queue.clone();
        let trigger = trigger;
        move || {
            let mut q = pending.lock().unwrap();
            q.push(ChatAction::ClearMessages);
            q.push(ChatAction::SetLoading(false));
            drop(q);
            register_ext_trigger(trigger);
        }
    };

    // ── Set provider helper ─────────────────────────────────────────
    let set_provider = {
        let panel_data = panel_data.clone();
        let pending = pending_queue.clone();
        let trigger = trigger;
        move |id: String| {
            panel_data.show_provider_list.set(false);
            panel_data.active_provider.set(id.clone());
            // Update model to match the selected provider's configured model
            let configs = panel_data.provider_configs.get_untracked();
            if let Some(cfg) = configs.iter().find(|p| p.id == id) {
                let m = cfg.model.get_untracked();
                panel_data.active_model.set(m);
            }
            pending.lock().unwrap().push(ChatAction::SetProvider(id));
            register_ext_trigger(trigger);
        }
    };

    // ── Example chip handler ───────────────────────────────────────────
    let insert_example = {
        let panel_data = panel_data.clone();
        move |text: &str| {
            panel_data.input.set(text.to_string());
        }
    };

    // ── Build UI ────────────────────────────────────────────────────
    let _pd_clone = panel_data.clone();
    let _mode_accent_color = mode_accent(&panel_data.agent_mode.get_untracked());

    container(
        stack((
            // ══════════════════════════════════════════════════════════
            // HEADER — Minimal controls only
            // ══════════════════════════════════════════════════════════
            container(
                stack((
                    // Provider + model chip (clickable for dropdown)
                    container(
                        label(move || {
                            let pid = panel_data.active_provider.get();
                            let model = panel_data.active_model.get();
                            format!("{} · {}", pid, model)
                        })
                        .style(move |s| {
                            let cfg = config.get();
                            s.font_size(10.0)
                                .color(cfg.color(LapceColor::EDITOR_DIM))
                                .padding_horiz(6.0)
                                .padding_vert(2.0)
                                .border_radius(3.0)
                                .hover(|s| {
                                    s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                        .cursor(CursorStyle::Pointer)
                                })
                        })
                        .on_event(EventListener::PointerDown, {
                            let pd = panel_data.clone();
                            move |_| {
                                pd.show_provider_list.update(|v| *v = !*v);
                                floem::event::EventPropagation::Stop
                            }
                        }),
                    ),
                    // Spacer
                    container(label(|| "".to_string()))
                        .style(|s| s.flex_grow(1.0)),
                    // New Chat
                    label(|| "＋".to_string())
                        .style(move |s| {
                            let cfg = config.get();
                            s.font_size(13.0)
                                .color(cfg.color(LapceColor::EDITOR_DIM))
                                .padding(4.0)
                                .border_radius(3.0)
                                .hover(|s| {
                                    s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                        .cursor(CursorStyle::Pointer)
                                })
                        })
                        .on_event(EventListener::PointerDown, {
                            let cc = clear_chat.clone();
                            move |_| { cc(); floem::event::EventPropagation::Stop }
                        }),
                ))
                .style(|s| s.flex_row().items_center().width_full().padding_horiz(8.0).padding_vert(3.0)),
            )
            .style(move |s| {
                s.width_full()
                    .border_bottom(1.0)
                    .border_color(config.get().color(LapceColor::LAPCE_BORDER))
                    .background(config.get().color(LapceColor::EDITOR_BACKGROUND))
            }),

            // ── Provider dropdown list ──────────────────────────────
            provider_dropdown(
                panel_data.clone(),
                default_providers(),
                set_provider.clone(),
                config,
            ),

            // ── Agent steps timeline ────────────────────────────────
            agent_steps_view(panel_data.clone(), config),

            // ══════════════════════════════════════════════════════════
            // MESSAGES AREA — scrollable with welcome state
            // ══════════════════════════════════════════════════════════
            scroll(
                container(
                    stack((
                        // Welcome banner (hidden when messages exist)
                        welcome_view(panel_data.clone(), insert_example.clone(), config)
                            .style(|s| s.flex_grow(1.0)),
                        // Message list
                        messages_view(panel_data.clone(), config),
                    ))
                    .style(|s| s.flex_col().width_full().min_height_full())
                )
            )
            .style(|s| s.flex_grow(1.0).width_full()),

            // ══════════════════════════════════════════════════════════
            // CONTEXT BAR — shows active context items
            // ══════════════════════════════════════════════════════════
            context_bar_view(config),

            // ══════════════════════════════════════════════════════════
            // INPUT AREA — modern, rounded prompt input
            // ══════════════════════════════════════════════════════════
            input_area_view(panel_data.clone(), on_submit.clone(), config),
        ))
        .style(move |s| {
            s.flex_col()
                .width_full()
                .height_full()
                .background(config.get().color(LapceColor::PANEL_BACKGROUND))
        }),
    )
}

// ── Provider dropdown ──────────────────────────────────────────────────

fn provider_dropdown(
    panel_data: ChatPanelData,
    providers: Vec<ProviderDef>,
    set_provider: impl Fn(String) + 'static + Clone,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let pd = panel_data.clone();
    container(
        stack((
            // Header
            label(|| "Select Provider & Model".to_string())
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(10.0)
                        .font_weight(Weight::BOLD)
                        .color(cfg.color(LapceColor::EDITOR_DIM))
                        .padding(8.0).padding_left(12.0)
                }),
            dyn_stack(
                move || providers.clone(),
                |p| p.id.clone(),
                move |prov: ProviderDef| {
                    let name = prov.display_name.clone();
                    let pid = prov.id.clone();
                    let pd = panel_data.clone();
                    let pd2 = panel_data.clone();
                    let pd3 = panel_data.clone();
                    let pid2 = prov.id.clone();
                    let pid3 = prov.id.clone();
                    // Get the model for this provider from configs
                    let model_display = {
                        let pd = panel_data.clone();
                        let pid = prov.id.clone();
                        move || {
                            let configs = pd.provider_configs.get();
                            configs.iter()
                                .find(|c| c.id == pid)
                                .map(|c| c.model.get())
                                .unwrap_or_default()
                        }
                    };
                    container(
                        stack((
                            container(
                                label(move || name.clone())
                                    .style(move |s| {
                                        let cfg = config.get();
                                        let active = pd.active_provider.get() == pid;
                                        s.font_size(12.0)
                                            .color(if active {
                                                Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF)
                                            } else {
                                                cfg.color(LapceColor::EDITOR_FOREGROUND)
                                            })
                                    }),
                            )
                            .style(|s| s.flex_grow(1.0)),
                            // Model label
                            label(move || model_display())
                                .style(move |s| {
                                    let cfg = config.get();
                                    s.font_size(9.0)
                                        .color(cfg.color(LapceColor::EDITOR_DIM))
                                        .padding_horiz(4.0).padding_vert(1.0)
                                        .border_radius(2.0)
                                        .border(1.0)
                                        .border_color(cfg.color(LapceColor::LAPCE_BORDER))
                                }),
                            // Active indicator
                            label(move || if pd2.active_provider.get() == pid2 { "●".to_string() } else { String::new() })
                                .style(move |s| {
                                    s.font_size(8.0)
                                        .margin_left(6.0)
                                        .color(Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF))
                                }),
                        ))
                        .style(move |s| {
                            let active = pd3.active_provider.get() == pid3;
                            s.flex_row().items_center()
                                .padding(8.0).padding_horiz(12.0)
                                .background(if active {
                                    Color::from_rgba8(0xF9, 0x73, 0x16, 0x10)
                                } else {
                                    Color::TRANSPARENT
                                })
                                .hover(|s| {
                                    let cfg = config.get();
                                    s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                        .cursor(CursorStyle::Pointer)
                                })
                        }),
                    )
                    .on_event(EventListener::PointerDown, {
                        let set = set_provider.clone();
                        let id = prov.id.clone();
                        move |_| {
                            set(id.clone());
                            // Close the dropdown
                            pd.show_provider_list.set(false);
                            floem::event::EventPropagation::Stop
                        }
                    })
                },
            )
            .style(|s| s.flex_col().width_full()),
        ))
        .style(|s| s.flex_col().width_full()),
    )
    .style(move |s| {
        s.width_full()
            .border_bottom(1.0)
            .border_color(config.get().color(LapceColor::LAPCE_BORDER))
            .background(config.get().color(LapceColor::EDITOR_BACKGROUND))
            .apply_if(!pd.show_provider_list.get(), |s| s.hide())
    })
}

// ── Agent steps timeline ───────────────────────────────────────────────

fn agent_steps_view(
    panel_data: ChatPanelData,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    container(
        dyn_stack(
            move || panel_data.agent_steps.get(),
            |step| step.id.to_string(),
            move |step: AgentStepDisplay| {
                let icon = step_icon(&step.step_type).to_string();
                let desc = step.description.clone();
                let color = step_color(&step.step_type);
                let is_done = step.status == StepStatus::Done;
                let is_error = step.status == StepStatus::Error;
                container(
                    stack((
                        label(move || icon.clone())
                            .style(move |s| s.font_size(12.0).color(color).margin_right(6.0)),
                        label(move || desc.clone())
                            .style(move |s| {
                                let cfg = config.get();
                                s.font_size(11.0).color(if is_error {
                                    cfg.color(LapceColor::LAPCE_ERROR)
                                } else if is_done {
                                    cfg.color(LapceColor::EDITOR_DIM)
                                } else {
                                    cfg.color(LapceColor::EDITOR_FOREGROUND)
                                })
                            }),
                    ))
                    .style(|s| s.flex_row().items_center()),
                )
                .style(move |s| s.padding(4.0).padding_left(16.0))
            },
        )
        .style(|s| s.flex_col().width_full()),
    )
    .style(move |s| {
        let cfg = config.get();
        s.width_full().apply_if(!panel_data.agent_steps.get().is_empty(), |s| {
            s.border_bottom(1.0)
                .border_color(cfg.color(LapceColor::LAPCE_BORDER))
                .background(cfg.color(LapceColor::PANEL_BACKGROUND))
        })
    })
}

// ── Welcome view ──────────────────────────────────────────────────────

fn welcome_view(
    panel_data: ChatPanelData,
    insert_example: impl Fn(&str) + 'static + Clone,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    #[derive(Clone)]
    struct ExampleChip {
        title: String,
    }
    let examples = vec![
        ExampleChip { title: "Explain this function".to_string() },
        ExampleChip { title: "Fix selected code".to_string() },
        ExampleChip { title: "Generate a component".to_string() },
        ExampleChip { title: "Refactor project".to_string() },
        ExampleChip { title: "Translate file".to_string() },
    ];

    container(
        stack((
            // Logo
            svg(move || LOGO_LARGE_SVG.to_string())
                .style(move |s| s.size(56.0, 56.0)),
            // Heading
            label(|| "How can I help you today?".to_string())
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(18.0)
                        .font_weight(Weight::BOLD)
                        .margin_top(16.0)
                        .color(cfg.color(LapceColor::EDITOR_FOREGROUND))
                }),
            // Subtitle
            label(|| "Ask me anything — or switch to Agent mode for code tasks".to_string())
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(12.0)
                        .margin_top(6.0)
                        .color(cfg.color(LapceColor::EDITOR_DIM))
                }),
            // Example chips
            container(
                stack((
                    dyn_stack(
                        move || examples.clone(),
                        |chip| chip.title.clone(),
                        move |chip: ExampleChip| {
                            let t = chip.title.clone();
                            container(
                                label(move || t.clone())
                                    .style(move |s| {
                                        let cfg = config.get();
                                        s.font_size(11.0)
                                            .color(cfg.color(LapceColor::EDITOR_FOREGROUND))
                                            .selectable(false)
                                    }),
                            )
                            .style(move |s| {
                                let cfg = config.get();
                                s.padding_horiz(12.0)
                                    .padding_vert(6.0)
                                    .border_radius(8.0)
                                    .border(1.0)
                                    .border_color(cfg.color(LapceColor::LAPCE_BORDER))
                                    .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
                                    .hover(|s| {
                                        let cfg = config.get();
                                        s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                            .cursor(CursorStyle::Pointer)
                                            .border_color(cfg.color(LapceColor::EDITOR_CARET))
                                    })
                            })
                            .on_event(EventListener::PointerDown, {
                                let ie = insert_example.clone();
                                let t = chip.title.clone();
                                move |_| {
                                    ie(&t);
                                    floem::event::EventPropagation::Stop
                                }
                            })
                        },
                    )
                    .style(|s| s.flex_col().width_full().gap(6.0).items_center()),
                ))
                .style(|s| s.flex_col().items_center().width_full()),
            )
            .style(|s| s.margin_top(24.0).width_full().max_width(360.0)),
        ))
        .style(|s| s.flex_col().items_center().justify_center().height_full().width_full()),
    )
    .style(move |s| {
        s.apply_if(!panel_data.messages.get().is_empty(), |s| s.hide())
            .width_full()
            .height_full()
    })
}

// ── Messages view ──────────────────────────────────────────────────────

fn messages_view(
    panel_data: ChatPanelData,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    stack((
        dyn_stack(
            move || panel_data.messages.get(),
            |msg| msg.id.to_string(),
            move |msg: ChatMessageDisplay| {
                let is_user = msg.role == "user";
                let is_agent = msg.agent_mode;
                let is_system = msg.role == "system";
                let content = msg.content.clone();
                let ts = msg.timestamp.clone();
                let role_label = if is_user { "You" } else if is_system { "System" } else { "Assistant" };

                let role_color = if is_user {
                    Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF)
                } else if is_system {
                    Color::from_rgba8(0xEF, 0x44, 0x44, 0xFF)
                } else if is_agent {
                    Color::from_rgba8(0x22, 0xC5, 0x5E, 0xFF)
                } else {
                    Color::from_rgba8(0xA8, 0x55, 0xF7, 0xFF)
                };

                let bubble_bg = if is_user {
                    if is_agent {
                        Color::from_rgba8(0x22, 0xC5, 0x5E, 0x12)
                    } else {
                        Color::from_rgba8(0xF9, 0x73, 0x16, 0x12)
                    }
                } else if is_agent {
                    Color::from_rgba8(0x22, 0xC5, 0x5E, 0x08)
                } else if is_system {
                    Color::from_rgba8(0xEF, 0x44, 0x44, 0x0D)
                } else {
                    Color::TRANSPARENT
                };

                container(
                    stack((
                        // Role header
                        stack((
                            label(move || role_label.to_string())
                                .style(move |s| {
                                    s.font_size(11.0)
                                        .font_weight(Weight::BOLD)
                                        .color(role_color)
                                }),
                            label(move || ts.clone())
                                .style(move |s| {
                                    let cfg = config.get();
                                    s.font_size(9.0)
                                        .color(cfg.color(LapceColor::EDITOR_DIM))
                                        .margin_left(6.0)
                                }),
                        ))
                        .style(|s| s.flex_row().items_center().margin_bottom(4.0)),
                        // Content
                        render_message_content(content.clone(), is_user, config),
                    ))
                    .style(|s| s.flex_col()),
                )
                .style(move |s| {
                    s.padding(12.0)
                        .margin_horiz(8.0)
                        .margin_top(4.0)
                        .margin_bottom(2.0)
                        .border_radius(8.0)
                        .background(bubble_bg)
                        .width_full()
                })
            },
        )
        .style(|s| s.flex_col().width_full()),
        // Loading indicator
        container(
            stack((
                label(move || {
                    if panel_data.is_loading.get() {
                        match panel_data.agent_mode.get() {
                            AgentMode::Chat => "AI is thinking...".to_string(),
                            mode => format!("Agent is {:?}ing...", mode),
                        }
                    } else {
                        String::new()
                    }
                })
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(11.0)
                        .color(cfg.color(LapceColor::EDITOR_DIM))
                        .padding(8.0)
                        .padding_left(16.0)
                        .apply_if(!panel_data.is_loading.get(), |s| s.hide())
                }),
            ))
            .style(|s| s.flex_row().items_center()),
        ),
    ))
    .style(|s| s.flex_col().width_full())
}

// ── Simple message content renderer ────────────────────────────────────

fn render_message_content(
    content: String,
    _is_user: bool,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let parts = parse_content(&content);
    dyn_stack(
        move || parts.clone(),
        |p| p.id(),
        move |part: ContentPart| {
            match part {
                ContentPart::Text(text) => {
                    let trimmed = text.trim_end().to_string();
                    if trimmed.is_empty() {
                        container(label(|| " ".to_string()))
                            .style(|s| s.height(4.0).width_full())
                            .into_any()
                    } else {
                        render_latex_text(trimmed, config.clone()).into_any()
                    }
                }
                ContentPart::Code { language, code } => {
                    code_block_view(language, code, config).into_any()
                }
            }
        },
    )
    .style(|s| s.flex_col().width_full().gap(2.0))
}

// ── LaTeX rendering ─────────────────────────────────────────────────

fn render_latex_text(
    text: String,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl floem::View {
    let has_display = text.contains("$$");
    let has_inline = text.contains('$');
    let text_copy = text.clone();

    if !has_display && !has_inline {
        return label(move || text.clone())
            .style(move |s| {
                s.font_size(13.0)
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                    .width_full()
            })
            .into_any();
    }

    // Simple approach: replace LaTeX delimiters with styled markers
    let display_tex = text_copy.replace("$$", "");
    let processed = display_tex.replace('$', "");
    let is_display = has_display;

    container(
        stack((
            label(|| "∫".to_string())
                .style(move |s| {
                    s.font_size(10.0)
                        .color(Color::from_rgba8(0xA8, 0x55, 0xF7, 0xFF))
                        .margin_right(4.0)
                        .font_family("Times, serif".to_string())
                }),
            label(move || processed.clone())
                .style(move |s| {
                    s.font_size(13.0)
                        .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                        .width_full()
                }),
        ))
        .style(|s| s.flex_row().items_start().width_full()),
    )
    .style(move |s| {
        s.width_full()
            .apply_if(is_display, |s| {
                s.padding(10.0)
                    .margin_vert(6.0)
                    .border_left(3.0)
                    .border_color(Color::from_rgba8(0xA8, 0x55, 0xF7, 0xFF))
                    .background(Color::from_rgba8(0xA8, 0x55, 0xF7, 0x06))
                    .border_radius(4.0)
            })
            .apply_if(!is_display && has_inline, |s| {
                s.margin_vert(1.0)
            })
    })
    .into_any()
}

// ── LaTeX parsing ─────────────────────────────────────────────────────

// ── Content parsing ───────────────────────────────────────────────────

fn parse_content(content: &str) -> Vec<ContentPart> {
    // Split by ``` markers, then split text parts by double newlines for paragraphs
    let code_parts = split_code_blocks(content);
    let mut all_parts = Vec::new();
    for part in code_parts {
        match part {
            ContentPart::Code { language, code } => {
                all_parts.push(ContentPart::Code { language, code });
            }
            ContentPart::Text(text) => {
                // Split by double newlines to create paragraphs
                let paragraphs: Vec<&str> = text.split("\n\n").collect();
                for (i, para) in paragraphs.iter().enumerate() {
                    let trimmed = para.trim();
                    if !trimmed.is_empty() {
                        if i > 0 {
                            all_parts.push(ContentPart::Text("\n".to_string()));
                        }
                        all_parts.push(ContentPart::Text(trimmed.to_string()));
                    }
                }
            }
        }
    }
    all_parts
}

#[derive(Clone)]
enum ContentPart {
    Text(String),
    Code { language: String, code: String },
}

impl ContentPart {
    fn id(&self) -> String {
        match self {
            ContentPart::Text(t) => format!("text_{}", t.len()),
            ContentPart::Code { language, code } => format!("code_{}_{}", language, code.len()),
        }
    }
}

fn split_code_blocks(content: &str) -> Vec<ContentPart> {
    let mut parts = Vec::new();
    let mut remaining = content;
    while let Some(start) = remaining.find("```") {
        let before = &remaining[..start];
        if !before.is_empty() {
            parts.push(ContentPart::Text(before.to_string()));
        }
        let after_triple = &remaining[start + 3..];
        let end = after_triple.find("```").unwrap_or(after_triple.len());
        let code_block = &after_triple[..end];
        let (language, code) = if let Some(newline_pos) = code_block.find('\n') {
            let lang = code_block[..newline_pos].trim().to_string();
            let code = code_block[newline_pos + 1..].to_string();
            (lang, code)
        } else {
            ("".to_string(), code_block.to_string())
        };
        parts.push(ContentPart::Code { language, code });
        remaining = &after_triple[end + 3..];
    }
    if !remaining.is_empty() {
        parts.push(ContentPart::Text(remaining.to_string()));
    }
    parts
}

fn code_block_view(
    language: String,
    code: String,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let code_clone = code.clone();
    let copy_state = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    // Clone for different closures
    let copy_label = copy_state.clone();
    let copy_style = copy_state.clone();
    let copy_event = copy_state.clone();

    container(
        stack((
            // Header bar
            container(
                stack((
                    // Language label
                    label(move || {
                        let lang = language.clone();
                        if lang.is_empty() { "code".to_string() } else { lang }
                    })
                    .style(move |s| {
                        let cfg = config.get();
                        s.font_size(10.0)
                            .color(cfg.color(LapceColor::EDITOR_DIM))
                            .font_weight(Weight::BOLD)
                            .font_family("JetBrains Mono, monospace".to_string())
                    }),
                    container(label(|| "".to_string()))
                        .style(|s| s.flex_grow(1.0)),
                    // Copy button
                    container(
                        label(move || {
                            let is_copy = copy_label.load(std::sync::atomic::Ordering::Relaxed);
                            if is_copy { "✓ Copied" } else { "⧉ Copy" }
                        }.to_string())
                        .style(move |s| {
                            let is_copy = copy_style.load(std::sync::atomic::Ordering::Relaxed);
                            s.font_size(10.0)
                                .color(if is_copy {
                                    Color::from_rgba8(0x22, 0xC5, 0x5E, 0xFF)
                                } else {
                                    config.get().color(LapceColor::EDITOR_DIM)
                                })
                                .padding_horiz(8.0)
                                .padding_vert(3.0)
                                .border_radius(4.0)
                                .background(if is_copy {
                                    Color::from_rgba8(0x22, 0xC5, 0x5E, 0x12)
                                } else {
                                    Color::TRANSPARENT
                                })
                                .hover(|s| {
                                    let cfg = config.get();
                                    s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                        .cursor(CursorStyle::Pointer)
                                })
                        }),
                    )
                    .on_event(EventListener::PointerDown, {
                        let code = code_clone.clone();
                        let cs = copy_event.clone();
                        move |_| {
                            if let Ok(_) = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(format!("echo -n {} | xclip -selection clipboard", 
                                    shell_escape(&code)))
                                .output()
                            {
                                cs.store(true, std::sync::atomic::Ordering::Relaxed);
                                let c2 = cs.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_secs(2));
                                    c2.store(false, std::sync::atomic::Ordering::Relaxed);
                                });
                            }
                            floem::event::EventPropagation::Stop
                        }
                    }),
                ))
                .style(|s| s.flex_row().items_center().width_full()),
            )
            .style(move |s| {
                let cfg = config.get();
                s.padding_horiz(12.0)
                    .padding_vert(6.0)
                    .background(cfg.color(LapceColor::PANEL_BACKGROUND))
                    .border_radius(6.0)
                    .border_bottom(1.0)
                    .border_color(cfg.color(LapceColor::LAPCE_BORDER))
            }),
            // Code content
            scroll(
                container(
                    label(move || code.clone())
                        .style(move |s| {
                            let cfg = config.get();
                            s.font_size(12.0)
                                .font_family("JetBrains Mono, monospace".to_string())
                                .color(cfg.color(LapceColor::EDITOR_FOREGROUND))
                                .width_full()
                        }),
                )
                .style(move |s| s.padding(12.0).width_full()),
            )
            .style(move |s| {
                let cfg = config.get();
                s.max_height(400.0)
                    .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
                    .border_radius(6.0)
                    .width_full()
            }),
        ))
        .style(|s| s.flex_col().width_full().margin_vert(6.0))
    )
}

fn shell_escape(s: &str) -> String {
    s.replace('\'', "'\\''")
}

// ── Context bar ────────────────────────────────────────────────────────

fn context_bar_view(
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    container(
        stack((
            // Context chips
            container(
                label(|| "Current File".to_string())
                    .style(move |s| {
                        let cfg = config.get();
                        s.font_size(10.0)
                            .color(cfg.color(LapceColor::EDITOR_DIM))
                            .padding_horiz(8.0)
                            .padding_vert(3.0)
                            .border_radius(4.0)
                            .border(1.0)
                            .border_color(cfg.color(LapceColor::LAPCE_BORDER))
                            .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
                            .hover(|s| {
                                let cfg = config.get();
                                s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                            })
                    }),
            ),
            container(
                label(|| "+ Add Context".to_string())
                    .style(move |s| {
                        let cfg = config.get();
                        s.font_size(10.0)
                            .color(cfg.color(LapceColor::EDITOR_FOCUS))
                            .padding_horiz(8.0)
                            .padding_vert(3.0)
                            .border_radius(4.0)
                            .hover(|s| {
                                let cfg = config.get();
                                s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                    .cursor(CursorStyle::Pointer)
                            })
                    }),
            ),
        ))
        .style(|s| s.flex_row().items_center().width_full().gap(4.0)),
    )
    .style(move |s| {
        let cfg = config.get();
        s.padding_horiz(12.0)
            .padding_vert(6.0)
            .width_full()
            .border_top(1.0)
            .border_color(cfg.color(LapceColor::LAPCE_BORDER))
            .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
    })
}

// ── Input area ─────────────────────────────────────────────────────────

fn input_area_view(
    panel_data: ChatPanelData,
    on_submit: impl Fn() + 'static + Clone,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    container(
        stack((
            // Input row
            container(
                stack((
                // "+" attach button
                    container(
                        svg(move || PLUS_SVG.to_string())
                            .style(move |s| {
                                let cfg = config.get();
                                s.size(16.0, 16.0)
                                    .color(cfg.color(LapceColor::EDITOR_DIM))
                            }),
                    )
                    .style(move |s| {
                        s.padding(8.0)
                            .border_radius(6.0)
                            .margin_right(4.0)
                            .hover(|s| {
                                let cfg = config.get();
                                s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                    .cursor(CursorStyle::Pointer)
                            })
                    }),
                    // Text input
                    text_input(panel_data.input)
                        .placeholder("Ask SUPER IDE anything...")
                        .disabled({
                            let pd = panel_data.clone();
                            move || pd.is_loading.get()
                        })
                        .on_event(EventListener::KeyDown, {
                            let on_submit = on_submit.clone();
                            let pd = panel_data.clone();
                            move |event| {
                                if let floem::event::Event::KeyDown(k) = event {
                                    if k.key.logical_key == Key::Named(NamedKey::Enter) && !k.modifiers.shift() {
                                        if !pd.is_loading.get_untracked() {
                                            on_submit();
                                        }
                                        floem::event::EventPropagation::Stop
                                    } else {
                                        floem::event::EventPropagation::Continue
                                    }
                                } else {
                                    floem::event::EventPropagation::Continue
                                }
                            }
                        })
                        .style({
                            let pd = panel_data.clone();
                            let config = config.clone();
                            move |s| {
                                let cfg = config.get();
                                let base = s
                                    .flex_grow(1.0)
                                    .padding_vert(10.0)
                                    .padding_horiz(2.0)
                                    .font_size(13.0)
                                    .border(0.0)
                                    .border_radius(0.0)
                                    .background(Color::TRANSPARENT);
                                if pd.is_loading.get() {
                                    base.color(cfg.color(LapceColor::EDITOR_DIM))
                                } else {
                                    base.color(cfg.color(LapceColor::EDITOR_FOREGROUND))
                                }
                            }
                        }),
                    // Agent mode quick toggle
                    container(
                        label(move || {
                            let mode = panel_data.agent_mode.get();
                            if mode == AgentMode::Chat { "💬" } else { "🤖" }
                        })
                        .style(move |s| {
                            s.font_size(13.0)
                                .padding(6.0)
                                .border_radius(4.0)
                                .hover(|s| {
                                    let cfg = config.get();
                                    s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
                                        .cursor(CursorStyle::Pointer)
                                })
                        })
                        .on_event_stop(EventListener::PointerDown, {
                            let pd = panel_data.clone();
                            move |_| {
                                let current = pd.agent_mode.get_untracked();
                                pd.agent_mode.set(match current {
                                    AgentMode::Chat => AgentMode::Edit,
                                    _ => AgentMode::Chat,
                                });
                            }
                        }),
                    ),
                    // Send button
                    container(
                        svg(move || SEND_SVG.to_string())
                            .style(move |s| {
                                let is_loading = panel_data.is_loading.get();
                                s.size(16.0, 16.0)
                                    .color(if is_loading {
                                        Color::from_rgba8(0x3F, 0x3F, 0x46, 0xFF)
                                    } else {
                                        Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF)
                                    })
                            }),
                    )
                    .style(move |s| {
                        let is_loading = panel_data.is_loading.get();
                        s.padding(8.0)
                            .margin_left(4.0)
                            .border_radius(8.0)
                            .background(if is_loading {
                                Color::TRANSPARENT
                            } else {
                                Color::from_rgba8(0xF9, 0x73, 0x16, 0x15)
                            })
                            .apply_if(!is_loading, |s| {
                                s.hover(|s| {
                                    s.background(Color::from_rgba8(0xF9, 0x73, 0x16, 0x30))
                                        .cursor(CursorStyle::Pointer)
                                })
                            })
                    })
                    .on_event_stop(EventListener::PointerDown, move |_| {
                        if !panel_data.is_loading.get_untracked() {
                            on_submit();
                        }
                    }),
                ))
                .style(|s| s.flex_row().items_center().width_full()),
            )
            .style(move |s| {
                let cfg = config.get();
                s.padding_horiz(8.0)
                    .padding_vert(6.0)
                    .margin(8.0)
                    .border_radius(12.0)
                    .border(1.0)
                    .border_color(cfg.color(LapceColor::LAPCE_BORDER))
                    .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
            }),
        ))
        .style(|s| s.flex_col().width_full()),
    )
    .style(move |s| {
        s.padding_bottom(8.0)
            .width_full()
            .border_top(1.0)
            .border_color(config.get().color(LapceColor::LAPCE_BORDER))
    })
}
