use core::ptr;

// UART register addresses (based on ESP32-S3 datasheet)
const UART_FIFO_REG: *mut u32 = 0x60000000 as *mut u32; // FIFO Register
const UART_INT_RAW_REG: *mut u32 = 0x60000004 as *mut u32; // Interrupt Raw Register
const UART_INT_ENA_REG: *mut u32 = 0x6000000C as *mut u32; // Interrupt Enable Register
const UART_CLKDIV_REG: *mut u32 = 0x60000014 as *mut u32; // Clock Divider Register
const UART_CONF0_REG: *mut u32 = 0x60000020 as *mut u32; // Configuration Register 0
const UART_CONF1_REG: *mut u32 = 0x60000024 as *mut u32; // Configuration Register 1
const UART_STATUS_REG: *mut u32 = 0x6000001C as *mut u32; // Status Register

const SYSTEM_PERIP_CLK_EN0_REG: *mut u32 = 0x60000018 as *mut u32; // to access to USART RAM clock

const SYSTEM_UART_MEM_CLK_EN: u32 = 1 << 24; // to enable UART_MEM clock
const SYSTEM_UART_CLK_EN: u32 = 1 << 2; // to enable UART clock

const SYSTEM_PERIP_RST_EN0_REG: *mut u32 = 0x60000020 as *mut u32;
const SYSTEM_UART_RST: u32 = 1 << 2; // to reset UART


const UART_CLK_CONF_REG: *mut u32 = 0x60000078 as *mut u32; // clock configuration register

const UART_RST_CORE_REG: u32 = 1 << 23; // to reset UART TX/RX

const UART_ID_REG: *mut u32 = 0x60000080 as *mut u32; // UART ID config register

const UART_UPDATE_CTRL_REG: u32 = 1 << 31; // to synchronize registers to UART Core's clock domain



// UART initialization & configuration (according to the right datasheet)
pub fn usart_init(baud_rate: u32) {
    unsafe {

        // Resetting UARTn

        // first step : enabling UART RAM clock
        ptr::write_volatile(SYSTEM_PERIP_CLK_EN0_REG,SYSTEM_UART_MEM_CLK_EN);

        // set SYSTEM_UART_CLK_EN 
        ptr::write_volatile(SYSTEM_PERIP_CLK_EN0_REG,SYSTEM_UART_CLK_EN);

        // clear SYSTEM_UART_RST
        unsafe {
            let reg_val = ptr::read_volatile(SYSTEM_PERIP_RST_EN0_REG);
            ptr::write_volatile(SYSTEM_PERIP_RST_EN0_REG, reg_val & !SYSTEM_UART_RST);
        } 

        // writing UART_RST_CORE to 1
        ptr::write_volatile(UART_CLK_CONF_REG,UART_RST_CORE_REG);

        // writing SYSTEM_UART_RST to 1
        ptr::write_volatile(SYSTEM_PERIP_RST_EN0_REG,SYSTEM_UART_RST);

        // clear SYSTEM_UART_RST (yes a second time)
        unsafe {
            let reg_val = ptr::read_volatile(SYSTEM_PERIP_RST_EN0_REG);
            ptr::write_volatile(SYSTEM_PERIP_RST_EN0_REG, reg_val & !SYSTEM_UART_RST);
        } 

        // clear UART_RST_CORE
        unsafe {
            let reg_val = ptr::read_volatile(UART_CLK_CONF_REG);
            ptr::write_volatile(UART_CLK_CONF_REG, reg_val & !UART_RST_CORE_REG);
        } 
      

        // Clear UART_UPDATE_CTRL
        unsafe {
            let reg_val = ptr::read_volatile(UART_ID_REG);
            ptr::write_volatile(UART_ID_REG, reg_val & !UART_UPDATE_CTRL_REG);
        } 

        // Wait for UART_REG_UPDATE to become 0
        while ptr::read_volatile(UART_ID_REG) & (UART_UPDATE_CTRL_REG) != 0 {}

        // Configuring UART communication
        let clk_div = (80_000_000 / baud_rate) as u32;
        ptr::write_volatile(UART_CLKDIV_REG,clk_div); // Set baud rate

        // Configure parity, data length
        ptr::write_volatile(UART_CONF0_REG,(1 << 0) | (1 << 2));

        // Synchronize configured values
        ptr::write_volatile(UART_ID_REG,UART_UPDATE_CTRL_REG);
    }
}

// Sending a byte via UART
pub fn usart_send(data: u8) {
    unsafe {
        // Wait until TX FIFO is not full
        while (ptr::read_volatile(UART_STATUS_REG) & (1 << 1)) == 0 {}
        UART_FIFO_REG.write_volatile(data as u32); // Write data to TX FIFO
    }
}

// Receiving a byte via UART
pub fn usart_receive() -> u8 {
    unsafe {
        // Wait until RX FIFO has data
        while (ptr::read_volatile(UART_STATUS_REG) & (1 << 0)) == 0 {}
        UART_FIFO_REG.read_volatile() as u8 // Read data from RX FIFO
    }
}
