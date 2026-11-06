//! Hypervisor manager model.

use crate::vm::{VirtualMachine, VmState};

#[derive(Debug, Default)]
pub struct HypervisorManager {
    pub vms: Vec<VirtualMachine>,
}

impl HypervisorManager {
    pub fn new() -> Self { Self::default() }
    pub fn define_vm(&mut self, vm: VirtualMachine) { self.vms.push(vm); }
    pub fn start_vm(&mut self, name: &str) -> Result<(), String> {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.name == name) {
            vm.state = VmState::Running;
            Ok(())
        } else {
            Err("VM not found".into())
        }
    }
    pub fn stop_vm(&mut self, name: &str) -> Result<(), String> {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.name == name) {
            vm.state = VmState::Stopped;
            Ok(())
        } else {
            Err("VM not found".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VmConfig;

    #[test]
    fn hypervisor_manager() {
        let mut hm = HypervisorManager::new();
        hm.define_vm(VirtualMachine::new("test".into(), VmConfig { ram_mb: 4096, vcpus: 4, arch: "x86_64".into() }));
        assert!(hm.start_vm("test").is_ok());
    }
}
