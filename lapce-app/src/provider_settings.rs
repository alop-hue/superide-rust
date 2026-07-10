use std::rc::Rc;

use floem::{
    View,
    reactive::{RwSignal, Scope, SignalGet},
    views::{Decorators, dyn_stack, label, scroll, stack, text_input},
};

use crate::{
    config::color::LapceColor,
    window_tab::WindowTabData,
};

#[derive(Clone)]
pub struct ProviderConfigItem {
    pub id: String,
    pub display_name: String,
    pub api_key: RwSignal<String>,
    pub endpoint: RwSignal<String>,
    pub model: RwSignal<String>,
}

pub struct ProviderSettingsData {
    pub providers: RwSignal<Vec<ProviderConfigItem>>,
}

impl ProviderSettingsData {
    pub fn new(cx: Scope) -> Self {
        let providers = cx.create_rw_signal(vec![
            ProviderConfigItem {
                id: "openai".to_string(),
                display_name: "OpenAI".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://api.openai.com/v1".to_string()),
                model: cx.create_rw_signal("gpt-4o".to_string()),
            },
            ProviderConfigItem {
                id: "anthropic".to_string(),
                display_name: "Anthropic".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://api.anthropic.com/v1".to_string()),
                model: cx.create_rw_signal("claude-sonnet-4-20250514".to_string()),
            },
            ProviderConfigItem {
                id: "openrouter".to_string(),
                display_name: "OpenRouter".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://openrouter.ai/api/v1".to_string()),
                model: cx.create_rw_signal("openai/gpt-4o".to_string()),
            },
            ProviderConfigItem {
                id: "ollama".to_string(),
                display_name: "Ollama (Local)".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("http://localhost:11434".to_string()),
                model: cx.create_rw_signal("llama3.2".to_string()),
            },
            ProviderConfigItem {
                id: "deepseek".to_string(),
                display_name: "DeepSeek".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://api.deepseek.com".to_string()),
                model: cx.create_rw_signal("deepseek-chat".to_string()),
            },
            ProviderConfigItem {
                id: "qwen".to_string(),
                display_name: "Qwen".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://dashscope.aliyuncs.com/compatible-mode/v1".to_string()),
                model: cx.create_rw_signal("qwen-plus".to_string()),
            },
            ProviderConfigItem {
                id: "grok".to_string(),
                display_name: "Grok (xAI)".to_string(),
                api_key: cx.create_rw_signal(String::new()),
                endpoint: cx.create_rw_signal("https://api.x.ai/v1".to_string()),
                model: cx.create_rw_signal("grok-3".to_string()),
            },
        ]);
        Self { providers }
    }
}

pub fn provider_settings_view(
    window_tab_data: Rc<WindowTabData>,
) -> impl View {
    let config = window_tab_data.common.config;
    let data = ProviderSettingsData::new(window_tab_data.common.scope);

    stack((
        label(|| "AI Providers".to_string())
            .style(move |s| {
                s.font_bold()
                    .font_size(16.0)
                    .padding(16.0)
                    .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
            }),
        scroll(
            dyn_stack(
                move || data.providers.get(),
                |p| p.id.clone(),
                move |prov: ProviderConfigItem| {
                    let name = prov.display_name.clone();
                    stack((
                        label(move || name.clone())
                            .style(move |s| {
                                let c = config.get();
                                s.font_size(14.0)
                                    .font_bold()
                                    .color(c.color(LapceColor::EDITOR_FOREGROUND))
                            }),
                        label(|| "API Key".to_string())
                            .style(move |s| {
                                let c = config.get();
                                s.font_size(11.0)
                                    .color(c.color(LapceColor::EDITOR_DIM))
                                    .margin_top(8.0)
                            }),
                        text_input(prov.api_key)
                            .style(move |s| {
                                let c = config.get();
                                s.width_pct(100.0)
                                    .padding(8.0)
                                    .border(1.0)
                                    .border_radius(4.0)
                                    .border_color(c.color(LapceColor::LAPCE_BORDER))
                                    .background(c.color(LapceColor::EDITOR_BACKGROUND))
                                    .color(c.color(LapceColor::EDITOR_FOREGROUND))
                            }),
                        label(|| "Endpoint".to_string())
                            .style(move |s| {
                                let c = config.get();
                                s.font_size(11.0)
                                    .color(c.color(LapceColor::EDITOR_DIM))
                                    .margin_top(8.0)
                            }),
                        text_input(prov.endpoint)
                            .style(move |s| {
                                let c = config.get();
                                s.width_pct(100.0)
                                    .padding(8.0)
                                    .border(1.0)
                                    .border_radius(4.0)
                                    .border_color(c.color(LapceColor::LAPCE_BORDER))
                                    .background(c.color(LapceColor::EDITOR_BACKGROUND))
                                    .color(c.color(LapceColor::EDITOR_FOREGROUND))
                            }),
                        label(|| "Model".to_string())
                            .style(move |s| {
                                let c = config.get();
                                s.font_size(11.0)
                                    .color(c.color(LapceColor::EDITOR_DIM))
                                    .margin_top(8.0)
                            }),
                        text_input(prov.model)
                            .style(move |s| {
                                let c = config.get();
                                s.width_pct(100.0)
                                    .padding(8.0)
                                    .border(1.0)
                                    .border_radius(4.0)
                                    .border_color(c.color(LapceColor::LAPCE_BORDER))
                                    .background(c.color(LapceColor::EDITOR_BACKGROUND))
                                    .color(c.color(LapceColor::EDITOR_FOREGROUND))
                            }),
                    ))
                    .style(move |s| {
                        let c = config.get();
                        s.flex_col()
                            .padding(12.0)
                            .margin(8.0)
                            .margin_bottom(4.0)
                            .border_radius(6.0)
                            .background(c.color(LapceColor::PANEL_BACKGROUND))
                            .border(1.0)
                            .border_color(c.color(LapceColor::LAPCE_BORDER))
                    })
                },
            )
            .style(|s| s.flex_col().width_full().padding(8.0))
        )
        .style(|s| s.flex_grow(1.0).width_full()),
    ))
    .style(move |s| {
        s.flex_col()
            .width_full()
            .height_full()
            .background(config.get().color(LapceColor::EDITOR_BACKGROUND))
    })
}
