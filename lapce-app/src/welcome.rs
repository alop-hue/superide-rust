use std::rc::Rc;
use std::sync::Arc;

use floem::{
    View,
    event::EventListener,
    peniko::Color,
    reactive::{ReadSignal, SignalGet},
    style::CursorStyle,
    views::{Decorators, container, h_stack, label, svg, v_stack},
};

use crate::config::{LapceConfig, color::LapceColor};
use crate::command::LapceWorkbenchCommand;
use crate::window_tab::WindowTabData;

const BRAND_ORANGE: Color = Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF);

pub fn welcome_view(
    window_tab_data: Rc<WindowTabData>,
) -> impl View {
    let workbench_command = window_tab_data.common.workbench_command;
    let config = window_tab_data.common.config;
    let workspace = window_tab_data.workspace.path.clone();

    container(
        v_stack((
            // ── Spacer ──────────────────────────────────────
            container(label(|| "".to_string())).style(|s| s.flex_grow(1.0)),

            // ── Logo ────────────────────────────────────────
            svg(move || crate::config::LOGO.to_string())
                .style(move |s| s.size(64.0, 64.0).margin_bottom(8.0)),

            // ── Hero ────────────────────────────────────────
            label(|| "SUPER IDE".to_string())
                .style(move |s| {
                    s.font_size(32.0)
                        .font_bold()
                        .color(BRAND_ORANGE)
                }),
            label(|| "Lightning-fast AI-powered code editor".to_string())
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(13.0)
                        .color(cfg.color(LapceColor::EDITOR_DIM))
                        .margin_top(4.0)
                }),

            // ── Action cards ─────────────────────────────────
            container(
                h_stack((
                    welcome_card(
                        "📁 Open Folder",
                        "Open a project folder\nto get started",
                        Color::from_rgba8(0x3B, 0x82, 0xF6, 0xFF),
                        config,
                        {
                            let cmd = workbench_command.clone();
                            move || cmd.send(LapceWorkbenchCommand::OpenFolder)
                        },
                    ),
                    welcome_card(
                        "📄 New File",
                        "Create a new\nuntitled file",
                        Color::from_rgba8(0x22, 0xC5, 0x5E, 0xFF),
                        config,
                        {
                            let cmd = workbench_command.clone();
                            move || cmd.send(LapceWorkbenchCommand::NewFile)
                        },
                    ),
                    welcome_card(
                        "🔍 Command Palette",
                        "Run commands and\nopen files",
                        Color::from_rgba8(0xA8, 0x55, 0xF7, 0xFF),
                        config,
                        {
                            let cmd = workbench_command.clone();
                            move || cmd.send(LapceWorkbenchCommand::PaletteCommand)
                        },
                    ),
                    welcome_card(
                        "⚙️ Open Settings",
                        "Customize SUPER IDE\nto your liking",
                        Color::from_rgba8(0xF9, 0x73, 0x16, 0xFF),
                        config,
                        {
                            let cmd = workbench_command.clone();
                            move || cmd.send(LapceWorkbenchCommand::OpenSettings)
                        },
                    ),
                ))
                .style(|s| s.gap(16.0).items_start()),
            )
            .style(|s| s.margin_top(32.0)),

            // ── Recent section ──────────────────────────────
            v_stack((
                welcome_recent_item(
                    "📂 Open a Recent Workspace",
                    "Browse and open recently used project folders",
                    config,
                    {
                        let cmd = workbench_command.clone();
                        move || cmd.send(LapceWorkbenchCommand::PaletteWorkspace)
                    },
                ),
                welcome_recent_item(
                    "🌐 Clone Git Repository",
                    "Clone a repository from GitHub, GitLab, or other remote",
                    config,
                    move || {},
                ),
            ))
            .style(|s| s.margin_top(24.0).width_pct(100.0).max_width(480.0)),

            // ── Toggle checkbox ─────────────────────────────
            container(
                h_stack((
                    label(move || {
                        let cfg = config.get();
                        if cfg.core.show_welcome { "☑" } else { "☐" }
                    }.to_string())
                    .style(move |s| {
                        s.font_size(14.0)
                            .color(BRAND_ORANGE)
                            .margin_right(6.0)
                    }),
                    label(|| "Show welcome screen on start".to_string())
                        .style(move |s| {
                            let cfg = config.get();
                            s.font_size(11.0)
                                .color(cfg.color(LapceColor::EDITOR_DIM))
                        }),
                ))
                .style(|s| s.items_center().cursor(CursorStyle::Pointer))
                .on_event_stop(EventListener::PointerDown, move |_| {
                    use serde::Serialize;
                    let new_val = !config.get_untracked().core.show_welcome;
                    if let Ok(value) = Serialize::serialize(
                        &new_val,
                        toml_edit::ser::ValueSerializer::new(),
                    ) {
                        LapceConfig::update_file("core", "show-welcome", value);
                    }
                }),
            )
            .style(|s| s.margin_top(32.0)),

            // ── Spacer ──────────────────────────────────────
            container(label(|| "".to_string())).style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.flex_col().items_center().size_pct(100.0, 100.0)),
    )
    .style(move |s| {
        let cfg = config.get();
        let has_workspace = workspace.is_some();
        let show_welcome = cfg.core.show_welcome;
        s.apply_if(!show_welcome || has_workspace, |s| s.hide())
            .width_full()
            .height_full()
            .items_center()
            .justify_center()
            .background(cfg.color(LapceColor::PANEL_BACKGROUND))
    })
}

fn welcome_card(
    title: &'static str,
    description: &'static str,
    accent: Color,
    config: ReadSignal<Arc<LapceConfig>>,
    on_click: impl Fn() + 'static,
) -> impl View {
    container(
        v_stack((
            label(move || title.to_string())
                .style(move |s| {
                    s.font_size(14.0)
                        .font_bold()
                        .color(accent)
                }),
            label(move || description.to_string())
                .style(move |s| {
                    let cfg = config.get();
                    s.font_size(11.0)
                        .color(cfg.color(LapceColor::EDITOR_DIM))
                        .margin_top(6.0)
                        .max_width(140.0)
                        .line_height(1.5)
                }),
        ))
        .style(|s| s.padding(16.0)),
    )
    .style(move |s| {
        let cfg = config.get();
        s.border(1.0)
            .border_radius(10.0)
            .border_color(cfg.color(LapceColor::LAPCE_BORDER))
            .background(cfg.color(LapceColor::EDITOR_BACKGROUND))
            .cursor(CursorStyle::Pointer)
            .width(160.0)
                    .hover(|s| {
                s.border_color(accent)
                    .background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND))
            })
    })
    .on_event_stop(EventListener::PointerDown, move |_| on_click())
}

fn welcome_recent_item(
    title: &'static str,
    subtitle: &'static str,
    config: ReadSignal<Arc<LapceConfig>>,
    on_click: impl Fn() + 'static,
) -> impl View {
    container(
        h_stack((
            container(
                label(|| "→".to_string())
                    .style(|s| s.font_size(14.0)),
            )
            .style(|s| s.margin_right(12.0)),
            v_stack((
                label(move || title.to_string())
                    .style(move |s| {
                        let cfg = config.get();
                        s.font_size(13.0).color(cfg.color(LapceColor::EDITOR_FOREGROUND))
                    }),
                label(move || subtitle.to_string())
                    .style(move |s| {
                        let cfg = config.get();
                        s.font_size(11.0).color(cfg.color(LapceColor::EDITOR_DIM))
                            .margin_top(2.0)
                    }),
            )),
        ))
        .style(move |s| {
            let cfg = config.get();
            s.padding(14.0)
                .padding_horiz(16.0)
                .border_radius(8.0)
                .width_full()
                .cursor(CursorStyle::Pointer)
                .hover(|s| s.background(cfg.color(LapceColor::PANEL_HOVERED_BACKGROUND)))
        }),
    )
    .on_event_stop(EventListener::PointerDown, move |_| on_click())
}
