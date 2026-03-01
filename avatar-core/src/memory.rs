pub struct GuestMemory {
    pub addr: *mut u8,  // pub 추가 (main.rs에서 접근 필요)
    size: usize,
}

impl GuestMemory {
    pub fn new(size: usize) -> Self {
        use libc::{mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};
        
        let addr = unsafe {
            mmap(
                std::ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0,
            )
        };
        
        if addr == libc::MAP_FAILED {
            panic!("mmap failed!");
        }
        
        Self { addr: addr as *mut u8, size }
    }
    
    pub fn load_payload(&mut self, data: &[u8]) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.addr,
                data.len(),
            );
        }
    }
}

impl Drop for GuestMemory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.addr as *mut _, self.size);
        }
    }
}