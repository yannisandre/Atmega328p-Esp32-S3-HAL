use core::ptr;

const UART_FIFO: *mut u32 = 0x60000000 as *mut u32; // FIFO Register
const UART_STATUS: *const u32 = 0x6000001C as *const u32; // Status Register
const UART_CONF0: *mut u32 = 0x60000020 as *mut u32; // Configuration Register
const UART_CLKDIV: *mut u32 = 0x60000014 as *mut u32; // Clock Divider Register

// initialize UART for ESP32-S3
pub fn usart_init(baud_rate: u32) {
    unsafe {
        let clk_div = (80_000_000 / baud_rate) as u32;
        ptr::write_volatile(UART_CLKDIV, clk_div);
        
        let conf0 = 0x03 << 2; // 1 byte
        ptr::write_volatile(UART_CONF0, conf0);
    }
}

/// sending a byte over uart
pub fn usart_send(data: u8) {
    unsafe {
        // emptying the TX FIFO
        while ptr::read_volatile(UART_STATUS) & (1 << 16) != 0 {} // TX FIFO full
        ptr::write_volatile(UART_FIFO, data as u32);
    }
}

/// receive a byte threw uart
pub fn usart_receive() -> u8 {
    unsafe {
        // Wait until the RX FIFO has some bytes
        while ptr::read_volatile(UART_STATUS) & (1 << 0) == 0 {} // RX FIFO empty
        ptr::read_volatile(UART_FIFO) as u8
    }
}
