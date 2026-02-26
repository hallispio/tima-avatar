use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

fn main() {
    println!("🚀 [Tima Avatar] Initiating KVM sequence...");

    // 1. KVM 디바이스 파일 열기 (하드웨어 통로 개척)
    let kvm = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("❌ [FATAL] KVM 통로 개방 실패. (권한 없거나 모듈 안 뜸)");

    // 2. KVM_GET_API_VERSION (매직 넘버 0xAE00) 날려서 버전 확인
    let version = unsafe { libc::ioctl(kvm.as_raw_fd(), 0xAE00, 0) };

    if version == 12 {
        println!("🔥 [SUCCESS] KVM API Version: {} (System Ready)", version);
        println!("🛡️ [NEXT] Prepare to inject Custom Hypercall (0x99)...");
    } else {
        println!("⚠️ [WARN] KVM API Version: {} (Unexpected)", version);
    }
}
