use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use superide_sdk::event::{EventSink, EventSubscription, EventBus as EventBusTrait, SuperEvent};

type Handler = Box<dyn Fn(SuperEvent) + Send + Sync>;

#[allow(dead_code)]
struct Subscription {
    id: u64,
    topic: String,
    handler: Handler,
}

impl Drop for Subscription {
    fn drop(&mut self) {
    }
}

impl EventSubscription for Subscription {
    fn cancel(self: Box<Self>) {
        drop(self);
    }
}

pub struct EventBus {
    subscriptions: Arc<RwLock<HashMap<String, Vec<(u64, Handler)>>>>,
    next_id: Arc<parking_lot::Mutex<u64>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(parking_lot::Mutex::new(1)),
        }
    }
}

impl EventSink for EventBus {
    fn publish(&self, event: SuperEvent) {
        let topic = event_topic(&event);
        let subs = self.subscriptions.read();
        if let Some(handlers) = subs.get(&topic) {
            for (_, handler) in handlers {
                handler(event.clone());
            }
        }
    }
}

impl EventBusTrait for EventBus {
    fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(SuperEvent) + Send + Sync>,
    ) -> Box<dyn EventSubscription> {
        let mut subs = self.subscriptions.write();
        let mut id_lock = self.next_id.lock();
        let id = *id_lock;
        *id_lock += 1;
        subs.entry(topic.to_string())
            .or_default()
            .push((id, handler));
        Box::new(Subscription {
            id,
            topic: topic.to_string(),
            handler: Box::new(|_| {}),
        })
    }
}

pub fn event_topic(event: &SuperEvent) -> String {
    match event {
        SuperEvent::WorkspaceOpened { .. } => "workspace.opened".to_string(),
        SuperEvent::WorkspaceClosed { .. } => "workspace.closed".to_string(),
        SuperEvent::FileOpened { .. } => "file.opened".to_string(),
        SuperEvent::FileSaved { .. } => "file.saved".to_string(),
        SuperEvent::CommandInvoked { .. } => "command.invoked".to_string(),
        SuperEvent::SearchRequested { .. } => "search.requested".to_string(),
        SuperEvent::TerminalStarted { .. } => "terminal.started".to_string(),
        SuperEvent::GitStatusChanged { .. } => "git.status_changed".to_string(),
        SuperEvent::AiRequested { .. } => "ai.requested".to_string(),
        SuperEvent::ProviderReady { .. } => "provider.ready".to_string(),
        SuperEvent::ProviderFailed { .. } => "provider.failed".to_string(),
        SuperEvent::AgentStarted { .. } => "agent.started".to_string(),
        SuperEvent::AgentFinished { .. } => "agent.finished".to_string(),
        SuperEvent::ExtensionActivated { .. } => "extension.activated".to_string(),
        SuperEvent::Custom { topic, .. } => topic.clone(),
    }
}