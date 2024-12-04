// registers (from datasheet)
const UART_FIFO_REG: *mut u32 = 0x60000000 as *mut u32; //UART_FIFO_REG : FIFO Register
const UART_INT_RAW_REG: *mut u32 = 0x60000004 as *mut u32; //UART_FIFO_REG : Interrupt Raw Register
const UART_INT_ENA_REG: *mut u32 = 0x6000000C as *mut u32; //UART_INT_ENA_REG : Interrupt Enable Register
const UART_CLKDIV_REG: *mut u32 = 0x60000014 as *mut u32; //UART_CLKDIV_REG : Clock Divider Register
const UART_CONF0_REG: *mut u32 = 0x60000020 as *mut u32; //UART_CONF0_REG : Configuration Register 0
const UART_CONF1_REG: *mut u32 = 0x60000024 as *mut u32; //UART_CONF1_REG : Configuration Register 1
const UART_STATUS_REG: *mut u32 = 0x6000001C as *mut u32; //UART_STATUS_REG : Status Register

// initialize UART for ESP32-S3
pub fn usart_init(baud_rate: u32) {
    unsafe {
        let clk_div = (80_000_000 / baud_rate) as u32;
        UART_CLKDIV_REG.write_volatile(clk_div);
        UART_CONF0_REG.write_volatile((1 << 0) | (1 << 2));
    }
}

/// sending a byte over uart
pub fn usart_send(data: u8) {
    unsafe {
        // emptying the TX FIFO
        while (UART_STATUS_REG.read_volatile() & (1 << 1)) == 0 {} //  TX FIFO is full
        UART_FIFO_REG.write_volatile(data as u32);
    }
}

/// receive a byte threw uart
pub fn usart_receive() -> u8 {
    unsafe {
        // Wait until the RX FIFO has some bytes
        while (UART_STATUS_REG.read_volatile() & (1 << 0)) == 0 {}
        UART_FIFO_REG.read_volatile() as u8
    }
}
