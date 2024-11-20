use core::ptr;

const UBRR0H: *mut u8 = 0xC5 as *mut u8; // Baud rate register for usart (High)
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // Baud rate register for usart (Low)
// Control and status registers :
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // register (A)
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // register (B)
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // register (C)
const UDR0: *mut u8 = 0xC6 as *mut u8;   // data register for usart

/// Initialize USART for ATmega328P with the specified baud rate.
pub fn usart_init(baud_rate: u16) {
    unsafe {
        let ubrr = ((16_000_000 / (16 * baud_rate as u32)) - 1) as u16;
        ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
        ptr::write_volatile(UBRR0L, (ubrr & 0xFF) as u8);
        ptr::write_volatile(UCSR0B, 1 << 3 | 1 << 4); // Enable Tx & Rx
        ptr::write_volatile(UCSR0C, 1 << 1 | 1 << 2); // 8-bit data, 1 stop bit
    }
}

// transmit a single byte over usart
pub fn usart_send(data: u8) {
    unsafe {
        while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {} // Wait for empty buffer
        ptr::write_volatile(UDR0, data);
    }
}

// receive a single byte from usart
pub fn usart_receive() -> u8 {
    unsafe {
        while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {} // Wait for data
        ptr::read_volatile(UDR0)
    }
}
