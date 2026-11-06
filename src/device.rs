//! Virtual device model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind { Disk, Nic, Console, Usb }

#[derive(Debug, Clone)]
pub struct VirtualDevice {
    pub kind: DeviceKind,
    pub backend: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_device() {
        let d = VirtualDevice { kind: DeviceKind::Disk, backend: "qcow2".into() };
        assert_eq!(d.kind, DeviceKind::Disk);
    }
}
