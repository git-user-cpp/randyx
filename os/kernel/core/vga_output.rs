pub fn vga_output_message(_message: &[u8]) {
    const MAX_LEN: usize = 32;
    let mut message = _message;

    if message.len() > MAX_LEN {
        message = b"Too many bytes to output";
    }

    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in message.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}