use core::ptr;

const TWBR: *mut u8 = 0xB8 as *mut u8; // TWI Bit Rate Register
const TWSR: *mut u8 = 0xB9 as *mut u8; // TWI Status Register
const TWAR: *mut u8 = 0xBA as *mut u8; // TWI (Slave) Address Register
const TWDR: *mut u8 = 0xBB as *mut u8; // TWI Data Register
const TWCR: *mut u8 = 0xBC as *mut u8; // TWI Control Register
const TWCR_TWEN: u8 = 1 << 2; // Enable TWI
const TWCR_TWSTA: u8 = 1 << 5; // Generate START condition
const TWCR_TWSTO: u8 = 1 << 4; // Generate STOP condition
const TWCR_TWINT: u8 = 1 << 7; // Interrupt flag

pub struct AtmegaI2c;

impl AtmegaI2c {
    // Initialize the I2C with the chosen clock frequency.
    pub fn init(clock_speed: u32, cpu_freq: u32) {
        unsafe {
            let twbr_value = ((cpu_freq / clock_speed) - 16) / 2; // (datasheet formula)
            ptr::write_volatile(TWBR, twbr_value as u8);
            ptr::write_volatile(TWSR, 0); // Prescaler value = 1
            ptr::write_volatile(TWCR, TWCR_TWEN); // Enable TWI
        }
    }

    // generates a START condition
    pub fn start() {
        unsafe {
            ptr::write_volatile(TWCR, TWCR_TWSTA | TWCR_TWEN | TWCR_TWINT);
            Self::wait_for_complete();
        }
    }

    // generates a STOP condition.
    pub fn stop() {
        unsafe {
            ptr::write_volatile(TWCR, TWCR_TWSTO | TWCR_TWEN | TWCR_TWINT);
        }
    }

    // writes a byte to the I2C bus.
    pub fn write_byte(data: u8) {
        unsafe {
            ptr::write_volatile(TWDR, data);
            ptr::write_volatile(TWCR, TWCR_TWEN | TWCR_TWINT);
            Self::wait_for_complete();
        }
    }

    // reads a byte from the I2C bus, with an option to send an ACK or NACK.
    pub fn read_byte(ack: bool) -> u8 {
        unsafe {
            let control = if ack {
                TWCR_TWEN | TWCR_TWINT
            } else {
                TWCR_TWEN | TWCR_TWINT | (1 << 6) // NACK
            };
            ptr::write_volatile(TWCR, control);
            Self::wait_for_complete();
            ptr::read_volatile(TWDR)
        }
    }

    // waits for the TWINT flag to be set -> operation completion.
    fn wait_for_complete() {
        unsafe {
            while (ptr::read_volatile(TWCR) & TWCR_TWINT) == 0 {}
        }
    }
}
