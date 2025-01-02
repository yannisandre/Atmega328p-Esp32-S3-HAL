use core::ptr;

// Base address for the I2C peripheral
const I2C_BASE: *mut u32 = 0x60013000 as *mut u32;

// Register offsets for the I2C peripheral
const I2C_CTR_REG: *mut u32 = 0x60013004 as *mut u32;
const I2C_DATA_REG: *mut u32 = 0x6001301C as *mut u32;
const I2C_SR_REG: *mut u32 = 0x60013008 as *mut u32;
const I2C_COMD0_REG: *mut u32 = 0x60013058 as *mut u32;
const I2C_COMD1_REG: *mut u32 = 0x6001305C as *mut u32;

// Control register bits
const I2C_MS_MODE: u32 = 1 << 4;       // Master mode
const I2C_SLAVE_MODE: u32 = 0 << 4;    // Slave mode (clear the MS_MODE bit)
const I2C_TRANS_START: u32 = 1 << 5;   // Start transmission
const I2C_CLK_EN: u32 = 1 << 8;        // Clock enable

// Status register bits
const I2C_RESP_REC: u32 = 1 << 0;      // ACK/NACK received
const I2C_ARB_LOST: u32 = 1 << 3;      // Arbitration lost
const I2C_BUS_BUSY: u32 = 1 << 4;      // Bus busy

pub struct Esp32I2c;

impl Esp32I2c {
    // Initialize the I2C peripheral in either Master or Slave mode
    // is_master is a boolean indicating the mode. true for master, false for slave.
    pub fn init(is_master: bool) {
        unsafe {
            let mut control = I2C_CLK_EN;
            if is_master {
                control |= I2C_MS_MODE;  // Set to master mode
            } else {
                control |= I2C_SLAVE_MODE;  // Set to slave mode
            }
            ptr::write_volatile(I2C_CTR_REG, control);
        }
    }

    // Generate a START condition
    pub fn start() {
        unsafe {
            // Configure the command register to generate a START condition
            let command = (6 << 11); // 6 = RSTART opcode
            ptr::write_volatile(I2C_COMD0_REG, command);
            Self::wait_for_completion();
        }
    }

    // Generate a STOP condition
    pub fn stop() {
        unsafe {
            // Configure the command register to generate a STOP condition
            let command = (2 << 11); // 2 = STOP opcode
            ptr::write_volatile(I2C_COMD0_REG, command);
            Self::wait_for_completion();
        }
    }

    // Write a byte to the I2C bus
    pub fn write_byte(data: u8) {
        unsafe {
            // starting transmission
            ptr::write_volatile(I2C_CTR_REG, I2C_TRANS_START);
            // Load data into the TX FIFO
            ptr::write_volatile(I2C_DATA_REG, data as u32);
            // Issue a WRITE command
            let command = (1 << 11) | 1; // 1 = WRITE opcode, 1 byte to write
            ptr::write_volatile(I2C_COMD0_REG, command);
            Self::wait_for_completion();
        }
    }

    //  Read a byte from the I2C bus
    pub fn read_byte(ack: bool) -> u8 {
        unsafe {
            // Configure the READ command with ACK or NACK
            let command = if ack {
                (3 << 11) | 1 // 3 = READ opcode, 1 byte to read
            } else {
                (3 << 11) | 1 | (1 << 10) // NACK bit set
            };
            ptr::write_volatile(I2C_COMD0_REG, command);
            Self::wait_for_completion();
            // Read the received data
            ptr::read_volatile(I2C_DATA_REG) as u8
        }
    }

    //  Wait for the transaction to complete
    fn wait_for_completion() {
        unsafe {
            while ptr::read_volatile(I2C_SR_REG) & I2C_BUS_BUSY != 0 {}
            // Ensure the CMD_DONE bit is set
            while ptr::read_volatile(I2C_COMD0_REG) & (1 << 31) == 0 {}
        }
    }

    // Check for errors
    pub fn check_errors() -> bool {
        unsafe {
            let status = ptr::read_volatile(I2C_SR_REG);
            if status & I2C_ARB_LOST != 0 {
                return false;
            }
            true
        }
    }
}
