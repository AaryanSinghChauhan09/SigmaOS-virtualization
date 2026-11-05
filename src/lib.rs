#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmState { Stopped, Starting, Running, Paused }

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub name: String,
    pub ram_mb: u32,
    pub vcpus: u8,
    pub state: VmState,
}

#[derive(Debug, Default)]
pub struct HypervisorManager {
    vms: Vec<VirtualMachine>,
}

impl HypervisorManager {
    pub fn new() -> Self { Self::default() }
    pub fn define_vm(&mut self, name: &str, ram: u32, vcpus: u8) {
        self.vms.push(VirtualMachine {
            name: name.to_string(),
            ram_mb: ram,
            vcpus,
            state: VmState::Stopped,
        });
    }
    pub fn start_vm(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.name == name) {
            vm.state = VmState::Running;
            Ok(())
        } else {
            Err("VM not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vm_lifecycle_management() {
        let mut mgr = HypervisorManager::new();
        mgr.define_vm("sigmaos-workstation-1", 4096, 4);
        assert!(mgr.start_vm("sigmaos-workstation-1").is_ok());
    }
}
