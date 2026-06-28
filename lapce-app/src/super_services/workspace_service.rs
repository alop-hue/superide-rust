use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Clone)]
pub struct WorkspaceInfo {
    pub root: PathBuf,
    pub name: String,
}

pub struct WorkspaceService {
    current: Arc<RwLock<Option<WorkspaceInfo>>>,
    recent: Arc<RwLock<Vec<PathBuf>>>,
}

impl Default for WorkspaceService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceService {
    pub fn new() -> Self {
        Self {
            current: Arc::new(RwLock::new(None)),
            recent: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn open(&self, root: PathBuf) {
        let name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        {
            let mut recent = self.recent.write();
            recent.retain(|p| p != &root);
            recent.insert(0, root.clone());
            if recent.len() > 20 {
                recent.truncate(20);
            }
        }
        *self.current.write() = Some(WorkspaceInfo { root, name });
    }

    pub fn close(&self) {
        *self.current.write() = None;
    }

    pub fn current(&self) -> Option<WorkspaceInfo> {
        self.current.read().clone()
    }

    pub fn recent(&self) -> Vec<PathBuf> {
        self.recent.read().clone()
    }
}