//! Virtual machine model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState { Stopped, Starting, Running, Paused, Stopping }

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub ram_mb: u32,
    pub vcpus: u8,
    pub arch: String,
}

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub name: String,
    pub config: VmConfig,
    pub state: VmState,
    pub disks: Vec<String>,
}

impl VirtualMachine {
    pub fn new(name: String, config: VmConfig) -> Self {
        Self { name, config, state: VmState::Stopped, disks: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_lifecycle() {
        let mut vm = VirtualMachine::new("test".into(), VmConfig { ram_mb: 4096, vcpus: 4, arch: "x86_64".into() });
        vm.state = VmState::Running;
        assert_eq!(vm.state, VmState::Running);
    }
}
