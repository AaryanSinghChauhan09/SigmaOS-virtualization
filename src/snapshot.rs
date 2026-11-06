//! VM snapshot model.

#[derive(Debug, Clone)]
pub struct VmSnapshot {
    pub id: String,
    pub vm_name: String,
    pub timestamp: u64,
    pub size_mb: u32,
}

#[derive(Debug, Default)]
pub struct SnapshotManager {
    pub snapshots: Vec<VmSnapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self { Self::default() }
    pub fn create_snapshot(&mut self, s: VmSnapshot) { self.snapshots.push(s); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_manager() {
        let mut sm = SnapshotManager::new();
        sm.create_snapshot(VmSnapshot { id: "snap1".into(), vm_name: "test".into(), timestamp: 0, size_mb: 1000 });
        assert_eq!(sm.snapshots.len(), 1);
    }
}
