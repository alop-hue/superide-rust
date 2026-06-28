use serde::{Deserialize, Serialize};
use structdesc::FieldNames;

#[derive(FieldNames, Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CoreConfig {
    #[field_names(desc = "Enable modal editing (Vim like)")]
    pub modal: bool,
    #[field_names(desc = "Set the color theme of SUPER IDE")]
    pub color_theme: String,
    #[field_names(desc = "Set the icon theme of SUPER IDE")]
    pub icon_theme: String,
    #[field_names(
        desc = "Enable customised titlebar and disable OS native one (Linux, BSD, Windows)"
    )]
    pub custom_titlebar: bool,
    #[field_names(
        desc = "Only allow double-click to open files in the file explorer"
    )]
    pub file_explorer_double_click: bool,
    #[field_names(
        desc = "Enable auto-reload for the plugin when its configuration changes."
    )]
    pub auto_reload_plugin: bool,
    #[field_names(
        desc = "Show the welcome screen when no editor is open. Turn off to open to a blank editor area."
    )]
    pub show_welcome: bool,
}
