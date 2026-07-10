use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone, Default)]
pub struct UsageRecord {
    pub provider_id: String,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub struct TokenUsageTracker {
    records: Arc<RwLock<Vec<UsageRecord>>>,
    total_input: Arc<RwLock<u64>>,
    total_output: Arc<RwLock<u64>>,
}

impl Default for TokenUsageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenUsageTracker {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(Vec::new())),
            total_input: Arc::new(RwLock::new(0)),
            total_output: Arc::new(RwLock::new(0)),
        }
    }

    pub fn record(&self, record: UsageRecord) {
        *self.total_input.write() += record.input_tokens;
        *self.total_output.write() += record.output_tokens;
        self.records.write().push(record);
    }

    pub fn totals(&self) -> (u64, u64) {
        (*self.total_input.read(), *self.total_output.read())
    }

    pub fn history(&self) -> Vec<UsageRecord> {
        self.records.read().clone()
    }

    pub fn reset(&self) {
        self.records.write().clear();
        *self.total_input.write() = 0;
        *self.total_output.write() = 0;
    }
}
