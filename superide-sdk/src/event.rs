#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiSurface {
    Chat,
    Agent,
    InlineCompletion,
    ModelPicker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuperEvent {
    WorkspaceOpened {
        root: String,
    },
    WorkspaceClosed {
        root: String,
    },
    FileOpened {
        path: String,
    },
    FileSaved {
        path: String,
    },
    CommandInvoked {
        command: String,
    },
    SearchRequested {
        query: String,
    },
    TerminalStarted {
        terminal_id: String,
    },
    GitStatusChanged {
        workspace: String,
    },
    AiRequested {
        surface: AiSurface,
    },
    ProviderReady {
        provider_id: String,
    },
    ProviderFailed {
        provider_id: String,
        message: String,
    },
    AgentStarted {
        agent_id: String,
    },
    AgentFinished {
        agent_id: String,
    },
    ExtensionActivated {
        extension_id: String,
    },
    Custom {
        topic: String,
        payload: String,
    },
}

pub trait EventSink: Send + Sync {
    fn publish(&self, event: SuperEvent);
}

pub trait EventSubscription: Send {
    fn cancel(self: Box<Self>);
}

pub trait EventBus: EventSink {
    fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(SuperEvent) + Send + Sync>,
    ) -> Box<dyn EventSubscription>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_requested_event() {
        let event = SuperEvent::AiRequested {
            surface: AiSurface::Chat,
        };
        assert_eq!(
            event,
            SuperEvent::AiRequested {
                surface: AiSurface::Chat
            }
        );
    }

    #[test]
    fn test_custom_event() {
        let event = SuperEvent::Custom {
            topic: "test.topic".to_string(),
            payload: "test payload".to_string(),
        };
        match &event {
            SuperEvent::Custom { topic, payload } => {
                assert_eq!(topic, "test.topic");
                assert_eq!(payload, "test payload");
            }
            _ => panic!("Wrong event variant"),
        }
    }

    #[test]
    fn test_event_clone() {
        let event = SuperEvent::WorkspaceOpened {
            root: "/home/test".to_string(),
        };
        let cloned = event.clone();
        assert_eq!(event, cloned);
    }
}
