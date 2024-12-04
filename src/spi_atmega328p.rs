#![no_std]
#![no_main]

use core::ptr;

// Register addresses for ATmega328P
const DDRB: *mut u8 = 0x24 as *mut u8;   // Data Direction Register B
const PORTB: *mut u8 = 0x25 as *mut u8;  // Port B Data Register
const SPCR: *mut u8 = 0x2C as *mut u8;   // SPI Control Register
const SPSR: *mut u8 = 0x2D as *mut u8;   // SPI Status Register
const SPDR: *mut u8 = 0x2E as *mut u8;   // SPI Data Register

/// SPI modes
pub enum SpiMode {
    Master,
    Slave,
}

/// Initialize the SPI as Master or Slave.
/// 
/// - **Master**: Configures MOSI (PB3) and SCK (PB5) as outputs, and MISO (PB4) as input.
/// - **Slave**: Configures MISO (PB4) as output, and MOSI (PB3), SCK (PB5), and SS (PB2) as inputs.
pub fn spi_init(mode: SpiMode) {
    unsafe {
        match mode {
            SpiMode::Master => {
                // Set MOSI (PB3) and SCK (PB5) as outputs, others as inputs
                let mut ddrb_value = ptr::read_volatile(DDRB);
                ddrb_value |= (1 << 3) | (1 << 5); // PB3 (MOSI), PB5 (SCK)
                ddrb_value &= !(1 << 4);           // PB4 (MISO) as input
                ptr::write_volatile(DDRB, ddrb_value);

                // Enable SPI, set Master mode, and set clock rate to fck/16
                ptr::write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0));
            }
            SpiMode::Slave => {
                // Set MISO (PB4) as output, others as inputs
                let mut ddrb_value = ptr::read_volatile(DDRB);
                ddrb_value |= (1 << 4);            // PB4 (MISO) as output
                ddrb_value &= !((1 << 3) | (1 << 5) | (1 << 2)); // PB3, PB5, PB2 as inputs
                ptr::write_volatile(DDRB, ddrb_value);

                // Enable SPI in Slave mode
                ptr::write_volatile(SPCR, (1 << 6));
            }
        }
    }
}

/// Transmit and receive a byte in SPI.
/// 
/// - For **Master**: Sends the byte and waits for a response.
/// - For **Slave**: Waits for the master to send a byte, then responds with the given byte.
///
/// Returns the received byte.
pub fn spi_transceive(data: u8) -> u8 {
    unsafe {
        // Write data to SPI Data Register
        ptr::write_volatile(SPDR, data);

        // Wait until the transmission is complete (SPIF flag in SPSR)
        while ptr::read_volatile(SPSR) & (1 << 7) == 0 {}

        // Return the received data from SPDR
        ptr::read_volatile(SPDR)
    }
}

/// Receive a byte in Slave mode.
/// Waits until data is received and then reads the SPI Data Register.
///
/// Returns the received byte.
pub fn spi_receive() -> u8 {
    unsafe {
        // Wait for data reception to complete (SPIF flag in SPSR)
        while ptr::read_volatile(SPSR) & (1 << 7) == 0 {}

        // Read and return the received data from SPDR
        ptr::read_volatile(SPDR)
    }
}
