use std::collections::HashMap;

#[derive(Clone)]
pub struct ContributionRegistry {
    commands: HashMap<String, CommandEntry>,
    themes: Vec<ThemeEntry>,
    icon_themes: Vec<IconThemeEntry>,
    views: Vec<ViewEntry>,
    providers: Vec<String>,
    agents: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CommandEntry {
    pub id: String,
    pub title: String,
    pub extension_id: String,
}

#[derive(Debug, Clone)]
pub struct ThemeEntry {
    pub id: String,
    pub path: String,
    pub extension_id: String,
}

#[derive(Debug, Clone)]
pub struct IconThemeEntry {
    pub id: String,
    pub path: String,
    pub extension_id: String,
}

#[derive(Debug, Clone)]
pub struct ViewEntry {
    pub id: String,
    pub title: String,
    pub extension_id: String,
}

impl Default for ContributionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ContributionRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            themes: Vec::new(),
            icon_themes: Vec::new(),
            views: Vec::new(),
            providers: Vec::new(),
            agents: Vec::new(),
        }
    }

    pub fn register_extension(
        &mut self,
        extension_id: &str,
        contributions: &[superide_sdk::extension::Contribution],
    ) {
        for c in contributions {
            match c {
                superide_sdk::extension::Contribution::Command { id, title } => {
                    self.commands.insert(
                        id.clone(),
                        CommandEntry {
                            id: id.clone(),
                            title: title.clone(),
                            extension_id: extension_id.to_string(),
                        },
                    );
                }
                superide_sdk::extension::Contribution::Theme { id, path } => {
                    self.themes.push(ThemeEntry {
                        id: id.clone(),
                        path: path.clone(),
                        extension_id: extension_id.to_string(),
                    });
                }
                superide_sdk::extension::Contribution::IconTheme { id, path } => {
                    self.icon_themes.push(IconThemeEntry {
                        id: id.clone(),
                        path: path.clone(),
                        extension_id: extension_id.to_string(),
                    });
                }
                superide_sdk::extension::Contribution::SidebarView { id, title } => {
                    self.views.push(ViewEntry {
                        id: id.clone(),
                        title: title.clone(),
                        extension_id: extension_id.to_string(),
                    });
                }
                superide_sdk::extension::Contribution::Provider { id } => {
                    self.providers.push(id.clone());
                }
                superide_sdk::extension::Contribution::Agent { id } => {
                    self.agents.push(id.clone());
                }
                superide_sdk::extension::Contribution::SettingsSchema { .. } => {}
            }
        }
    }

    pub fn commands(&self) -> &HashMap<String, CommandEntry> {
        &self.commands
    }

    pub fn themes(&self) -> &[ThemeEntry] {
        &self.themes
    }

    pub fn icon_themes(&self) -> &[IconThemeEntry] {
        &self.icon_themes
    }

    pub fn views(&self) -> &[ViewEntry] {
        &self.views
    }

    pub fn providers(&self) -> &[String] {
        &self.providers
    }

    pub fn agents(&self) -> &[String] {
        &self.agents
    }

    pub fn find_command(&self, id: &str) -> Option<&CommandEntry> {
        self.commands.get(id)
    }
}
