# 🌌 Tima Avatar
### KVM 기반 가상화 제어 시스템

> **"Beyond the Kernel, Into the Hypervisor."** > 단순한 커널 모듈을 넘어, 하드웨어 가상화 기술(KVM)을 직접 제어하는 Rust 가상화 엔진입니다. 이는 **ChronoProof**가 지향하는 '물리적 제로 트러스트'의 최종 진화 형태입니다.

---

### 🚧 진행 중 (Work In Progress)
본 프로젝트는 QEMU나 libvirt를 배제하고 KVM ioctl을 직접 제어하며, 연구 및 실험을 목적으로 독자적인 가상화 레이어를 구축합니다.

### ⚡ 핵심 모듈 설계 (Core Logic)
* **Zero-Copy Memory Pool**: `mmap` 직접 제어를 통한 메모리 격리 및 오버헤드 제거
* **Custom Hypercall (0x99)**: 게스트와 호스트 사이의 독자적인 데이터 통신 규약 수립
* **Raw Binary Payload**: ELF 헤더를 제거한 순수 어셈블리 주입을 통한 정밀 제어

### 👤 Tima Avatar (The Identity)
> **"Synchronized virtualization in Rust. Zero-copy. Phantom-guest."**

* **Identity**: 가상화 레이어에서 하드웨어와 소프트웨어 사이의 동기화를 담당하는 고스트 객체
* **Philosophy**: 존재하되 감지되지 않으며, 데이터의 흐름을 물리적 레벨에서 관측함

---

### ✅ 현재 상태
* ✅ 환경 설정 완료 (VMware Virtual Platform)
* ✅ KVM API 버전 12 확인 완료
* ✅ **포트 I/O (0x99) 통신 성공 (최신 업데이트)**
* 🔄 커스텀 하이퍼콜 (0x99) - 진행 중

---

### 📊 개발 로그

#### **2026-03-01: I/O 트랩 성공**
* **Milestone**: 게스트-호스트 간 실시간 통신 채널 구축
* **Implementation**: 커스텀 `0x99` 포트 I/O 트랩 핸들러 구현
* **Verification**: 게스트 환경에서 호스트 터미널로의 문자열 출력 확인
* **Version Control**: `feat/first-io-success` 브랜치 생성

#### **2026-02-26: 기초 다지기**
* **Environment**: Ryzen 5 / 16GB RAM 기반 가상화 환경 구축
* **KVM Status**: KVM API 버전 12 및 "System Ready" 상태 확인
* **Hypercall Prep**: 하이퍼콜 0x99 및 통신 브릿지 아키텍처 설계 착수

---

### 🛠️ 기술 세부 사항
* **Language**: Rust (Edition 2021)
* **Interface**: Linux KVM API (ioctl)
* **Libraries**: `kvm-ioctls`, `kvm-bindings`, `libc`
* **Communication**: 커스텀 포트 I/O (0x99) 트랩
