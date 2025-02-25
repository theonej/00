use core::ptr;


pub fn print(message: &str) {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;

    for byte in message.as_bytes() {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
