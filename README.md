## 🌌 Tima Avatar
**KVM-based Virtualization Control System**

## [🚧 Work In Progress]
Building a custom Rust-based VMM from scratch, directly controlling KVM ioctl (bypassing QEMU/libvirt) and implementing custom hypercalls for research and experimentation.

Note: This is an experimental project under active development. The codebase and low‑level interactions may change as the design evolves.

### ⚡ Current Status
- ✅ Environment Setup Complete
- ✅ KVM API Version 12 Verified
- 🔄 Custom Hypercall (0x99) - In Progress

### 🚀 Quick Start
```bash
# 1. KVM availability 확인
ls -l /dev/kvm

# 2. Build (inside avatar-core)
cd avatar-core && cargo build

# 3. Run (Requires sudo for KVM access)
sudo ../target/debug/tima-avatar
```

# Expected: [Tima Avatar] KVM API Version: 12
### 📊 Development Log
**2026-02-26: Foundation**
- KVM environment verification
- API Version 12 confirmed
- Hypercall preparation initiated


### 🛠️ Technical Details
