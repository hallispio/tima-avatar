# 🌌 Tima Avatar
> **KVM-based Virtualization Control System**

하드웨어 가상화 기술(KVM)을 직접 제어하여 호스트와 게스트 사이의 경계를 허무는 Rust 기반 가상화 엔진입니다.

### ⚡ Current Status
* **✅ Environment Setup**: Complete (Bare-metal / Nested Virt enabled)
* **✅ KVM API Verification**: Version 12 Confirmed
* **🔄 Custom Hypercall (0x99)**: In Progress (Implementing Guest-to-Host Bridge)

---

### 🚀 Quick Start
```bash
# Verify KVM availability
ls -l /dev/kvm

# Check KVM version via Ghost-Shell-Tool
cargo run --bin tima-check
# Expected: [GHOST] KVM API Version: 12


🛠️ Technical Details
Layer: Ring -1 (Hypervisor Level)

Logic: Custom VCPU Loop & VM-Exit Handling

Bridge: Rust-C FFI for KVM IOCTLs
