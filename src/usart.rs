use core::ptr;

const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // Adjust to the actual address if needed
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
const UDR0: *mut u8 = 0xC6 as *mut u8;   // USART Data Register

// Function to initialize USART with a given baud rate
pub fn usart_init(baud_rate: u16) {
    unsafe {
        // Set the baud rate (16 MHz / (16 * baud_rate)) - 1
        let ubrr = ((16_000_000 / (16 * baud_rate as u32)) - 1) as u16;
        ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
        ptr::write_volatile(UBRR0L, (ubrr & 0xFF) as u8);

        // Enable transmitter and receiver
        ptr::write_volatile(UCSR0B, 1 << 3 | 1 << 4);

        // Set frame format: 8 data bits, 1 stop bit
        ptr::write_volatile(UCSR0C, 1 << 1 | 1 << 2);
    }
}

// Function to send a byte over USART
pub fn usart_send(data: u8) {
    unsafe {
        // Wait for empty transmit buffer
        while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {}

        // Put data into the buffer, sends the data
        ptr::write_volatile(UDR0, data);
    }
}

// Function to receive a byte over USART
pub fn usart_receive() -> u8 {
    unsafe {
        // Wait for data to be received
        while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {}

        // Get and return received data from buffer
        ptr::read_volatile(UDR0)
    }
}
