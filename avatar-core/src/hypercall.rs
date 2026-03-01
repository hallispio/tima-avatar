use kvm_ioctls::VcpuFd;
use std::io::{self, Write};

pub struct HypercallHandler;

impl HypercallHandler {
    pub fn new() -> Self {
        Self
    }
    
    pub fn handle(&self, vcpu: &VcpuFd) -> Result<(), Box<dyn std::error::Error>> {
        let mut regs = vcpu.get_regs()?;
        
        // 🔍 디버깅: 레지스터 값 출력
        eprintln!("[DEBUG] Hypercall detected! rax=0x{:x}, rbx=0x{:x}, rip=0x{:x}", 
                  regs.rax, regs.rbx, regs.rip);
        
        // 0x99 인증 체크
        if regs.rax == 0x99 {
            // rbx = 데이터 페이로드
            let data = regs.rbx as u8 as char;
            print!("{}", data);
            
            // 🔥 stdout flush (버퍼링 방지)
            io::stdout().flush().ok();
            
            // 시간 진행 (vmcall 3바이트)
            regs.rip += 3;
            vcpu.set_regs(&regs)?;
            
            eprintln!("[DEBUG] Character printed: '{}' (0x{:02x})", data, data as u8);
        } else {
            // Unknown hypercall
            eprintln!("[Tima] Unknown hypercall: 0x{:x}", regs.rax);
            
            regs.rip += 3;
            vcpu.set_regs(&regs)?;
        }
        
        Ok(())
    }
}