mod hypercall;
mod payload;
mod memory;

use kvm_ioctls::*;
use kvm_bindings::*;
use hypercall::HypercallHandler;

fn main() {
    println!("🚀 [Tima Avatar] Initiating KVM sequence...");
    
    // 1. KVM 초기화
    let kvm = Kvm::new().unwrap();
    let version = kvm.get_api_version();
    println!("🔥 [SUCCESS] KVM API Version: {} (System Ready)", version);
    
    let vm = kvm.create_vm().unwrap();
    let mut vcpu = vm.create_vcpu(0).unwrap();
    
    // 2. 메모리 (Zero-Copy Pool)
    let mut guest_mem = memory::GuestMemory::new(0x100000);
    
    // 🚨 FIX 1: KVM에 메모리 전입신고!
    let mem_region = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0x0,
        memory_size: 0x100000,
        userspace_addr: guest_mem.addr as u64,
        flags: 0,
    };
    unsafe { vm.set_user_memory_region(mem_region).unwrap(); }
    
    // 3. 페이로드 주입
println!("🛡️ [NEXT] Prepare to inject Custom Hypercall (0x99)...");
let payload = payload::build_hello_world();

// 🔍 페이로드 크기 확인
eprintln!("[DEBUG] Payload size: {} bytes", payload.len());

// 🔍 페이로드 처음 20바이트 hex dump
eprint!("[DEBUG] Payload hex: ");
for (i, byte) in payload.iter().take(20).enumerate() {
    eprint!("{:02x} ", byte);
}
eprintln!();

guest_mem.load_payload(&payload);

// 🔍 메모리 주입 확인 (처음 20바이트 읽기)
unsafe {
    eprint!("[DEBUG] Memory hex: ");
    for i in 0..20 {
        eprint!("{:02x} ", *(guest_mem.addr.add(i)));
    }
    eprintln!();
}
    
    // 4. VCPU 설정
    setup_vcpu(&vcpu, 0x0);
    
    // 5. 하이퍼콜 핸들러
    let handler = HypercallHandler::new();
    
    // 6. VM-Exit 루프
    print!("[Guest Output] ");
    loop {
        match vcpu.run().unwrap() {
    VcpuExit::IoOut(port, data) => {
        if port == 0x99 {
            print!("{}", data[0] as char);
        }
    }
    VcpuExit::Hlt => {
        println!("\n[Tima] Guest halted gracefully.");
        break;
    }
    exit => {
        eprintln!("[DEBUG] Exit: {:?}", exit);
        break;
    }
}
    }
}

// 🚨 FIX 4: 16비트 리얼 모드 유지, CS.base = 0 강제 고정
fn setup_vcpu(vcpu: &VcpuFd, entry: u64) {
    let mut sregs = vcpu.get_sregs().unwrap();
    
    // 🚩 핵심: 모든 세그먼트를 0번지 기반으로 강제 리셋
    sregs.cs.base = 0;
    sregs.cs.selector = 0;
    sregs.ds.base = 0;
    sregs.ds.selector = 0;
    sregs.es.base = 0;
    sregs.es.selector = 0;
    sregs.ss.base = 0;
    sregs.ss.selector = 0;
    sregs.fs.base = 0;
    sregs.fs.selector = 0;
    sregs.gs.base = 0;
    sregs.gs.selector = 0;
    
    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    // 0번지(entry)에서 무조건 시작!
    regs.rip = entry; 
    regs.rflags = 0x2; // 기본 플래그 비트 고정
    vcpu.set_regs(&regs).unwrap();
}