pub fn build_hello_world() -> Vec<u8> {
    let mut code = Vec::new();
    
    // 🔍 테스트: 포트 I/O로 게스트 실행 확인
    let emit_char = |c: u8| -> Vec<u8> {
        vec![
            0xb0, c,        // mov al, c
            0xe6, 0x99,     // out 0x99, al
        ]
    };
    
    code.extend(emit_char(b'H'));
    code.extend(emit_char(b'e'));
    code.extend(emit_char(b'l'));
    code.extend(emit_char(b'l'));
    code.extend(emit_char(b'o'));
    
    code.push(0xf4); // hlt
    code
}