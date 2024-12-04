use crate::gpio::{Gpio, PinMode};

// GPIO registers
const GPIO_ENABLE_REG: *mut u32 = 0x60004020 as *mut u32;
const GPIO_OUT_REG: *mut u32 = 0x60004004 as *mut u32;
const GPIO_IN_REG: *const u32 = 0x6000403C as *const u32;

pub struct Esp32Gpio {
    pub pin: u8, // numéro du pin
}

impl Gpio for Esp32Gpio {
    // configure le mode sortie ou entrée du pin
    fn set_mode(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Output => {
                    // mode sortie
                    GPIO_ENABLE_REG.write_volatile(
                        GPIO_ENABLE_REG.read_volatile() | (1 << self.pin),
                    );
                }
                PinMode::Input => {
                    // mode entrée
                    GPIO_ENABLE_REG.write_volatile(
                        GPIO_ENABLE_REG.read_volatile() & !(1 << self.pin),
                    );
                }
            }
        }
    }

    // configure l'état du pin HIGH ou LOW
    fn write(&self, value: bool) {
        unsafe {
            if value {
                // met le pin à HIGH
                GPIO_OUT_REG.write_volatile(
                    GPIO_OUT_REG.read_volatile() | (1 << self.pin),
                );
            } else {
                // met le pin à LOW
                GPIO_OUT_REG.write_volatile(
                    GPIO_OUT_REG.read_volatile() & !(1 << self.pin),
                );
            }
        }
    }

    // lit l'état du pin
    fn read(&self) -> bool {

        unsafe {
            // lit l'état du pin
            (GPIO_IN_REG.read_volatile() & (1 << self.pin)) != 0
        }
    }
}
