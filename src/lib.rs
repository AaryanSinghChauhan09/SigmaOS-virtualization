//! SigmaOS-virtualization — hypervisor model.
//!
//! Models VM lifecycle, CPU/memory/virtual disk configuration,
//  snapshot management, and basic hypervisor abstractions.
//! Deterministic and std-only (no actual hypervisor access).

pub mod vm;
pub mod hypervisor;
pub mod snapshot;
pub mod device;

pub use vm::{VirtualMachine, VmState, VmConfig};
pub use hypervisor::HypervisorManager;
pub use snapshot::{VmSnapshot, SnapshotManager};
pub use device::{VirtualDevice, DeviceKind};
