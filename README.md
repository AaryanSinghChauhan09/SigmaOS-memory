# SigmaOS Memory

This crate exports a `MemoryManager` and organizes APIs under `memory`, `swap`,
and `thermal` modules. The repository contains a broad set of memory-management
source files; this top-level API alone does not establish that the code is
integrated with a booted kernel, safely managing physical pages, or enforcing
thermal limits on hardware. Check each subsystem's implementation and platform
evidence before relying on it.

## License

See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).

