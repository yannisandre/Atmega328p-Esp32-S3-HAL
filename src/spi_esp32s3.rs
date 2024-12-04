use core::ptr;

const SPI_BASE: u32 = 0x60003000; // SPI Controller 0 base address
const SPI_CMD_REG: *mut u32 = (SPI_BASE + 0x0000) as *mut u32; // SPI command register
const SPI_SLAVE_REG: *mut u32 = (SPI_BASE + 0x00E0) as *mut u32; // SPI slavemode register
const SPI_W0_REG: *mut u32 = (SPI_BASE + 0x0098) as *mut u32; // SPI data buffer

// SPI Modes (Master or Slave)
pub enum SpiMode {
    Master,
    Slave,
}

// Initialize SPI as Master or Slave
pub fn spi_init(mode: SpiMode) {
    unsafe {
        match mode {
            SpiMode::Master => {
              
                ptr::write_volatile(SPI_SLAVE_REG, ptr::read_volatile(SPI_SLAVE_REG) & !(1 << 26) ); // enabling master mode (26th bit of SPI_SLAVE register set to 0)
            }
            SpiMode::Slave => {
              
                ptr::write_volatile(SPI_SLAVE_REG, (1 << 26)); // enabling slave mode (26th but of SPI_SLAVE set to 1)
            }
        }
    }
}

// Transmit and receive a byte in master mode

// returning the received byte.
pub fn spi_transceive(data: u8) -> u8 {
    unsafe {
        // write data to the buffer
        ptr::write_volatile(SPI_W0_REG, data as u32);

        // start transmission (set SPI_USR bit in SPI_CMD_REG)
        ptr::write_volatile(SPI_CMD_REG, (1 << 24)); // set SPI_USR (24th bit) of the SPI_CMD register to one to start the transaction

        // wait for the SPI transaction to complete
        while ptr::read_volatile(SPI_CMD_REG) & (1 << 24) != 0 {} // Wait until SPI_USR bit clears (0)

        // read and return received data
        ptr::read_volatile(SPI_W0_REG) as u8
    }
}

// receiving a byte in Slave mode
// returning the received byte.
pub fn spi_receive() -> u8 {
    unsafe {
       
        while ptr::read_volatile(SPI_CMD_REG) & (1 << 24) == 0 {} // while the transaction is not started we wait

        // then we read and return the received data
        ptr::read_volatile(SPI_W0_REG) as u8
    }
}
