# SigmaOS Virtualization

This crate exports a `VirtualizationManager` and contains VM and virtualization
modules, including Firecracker and cgroup-related source. These code surfaces
do not establish that a VM can be safely launched, confined, or stopped on a
host. Validate each backend against its runtime, privilege boundary, resource
limits, and cleanup behavior before relying on it.

## License

See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).

