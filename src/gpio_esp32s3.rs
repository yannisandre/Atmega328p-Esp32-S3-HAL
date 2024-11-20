use crate::gpio::{Gpio, PinMode};

// adresse de base pour l'ESP32-S3
const GPIO_BASE: usize = 0x60004000; 

// offset pour les registre GPIO
const GPIO_ENABLE_W1TS_OFFSET: usize = 0x24; // port utiliser pour mettre un pin en tant que "sortie"
const GPIO_ENABLE_W1TC_OFFSET: usize = 0x28; // port utiliser pour mettre un pin en tant que "entrée"
const GPIO_OUT_W1TS_OFFSET: usize = 0x08;    // port utiliser pour mettre un pin à l'état "HIGH"
const GPIO_OUT_W1TC_OFFSET: usize = 0x0C;    // port utiliser pour mettre un pin à l'état "LOW"
const GPIO_IN_OFFSET: usize = 0x3C;          // port utiliser pour lire l'état d'un pin

pub struct Esp32Gpio {
    pub pin: u8, // numéro du pin
}

impl Gpio for Esp32Gpio {
    // configure le mode sortie ou entrée du pin
    fn set_mode(&self, mode: PinMode) {
        let gpio_enable_w1ts = unsafe { (GPIO_BASE + GPIO_ENABLE_W1TS_OFFSET) as *mut u32 };
        let gpio_enable_w1tc = unsafe { (GPIO_BASE + GPIO_ENABLE_W1TC_OFFSET) as *mut u32 };

        unsafe {
            match mode {
                PinMode::Output => {
                    // mode sortie
                    *gpio_enable_w1ts = 1 << self.pin;
                }
                PinMode::Input => {
                    // mode entrée
                    *gpio_enable_w1tc = 1 << self.pin;
                }
            }
        }
    }

    // configure l'état du pin HIGH ou LOW
    fn write(&self, value: bool) {
        let gpio_out_w1ts = unsafe { (GPIO_BASE + GPIO_OUT_W1TS_OFFSET) as *mut u32 };
        let gpio_out_w1tc = unsafe { (GPIO_BASE + GPIO_OUT_W1TC_OFFSET) as *mut u32 };

        unsafe {
            if value {
                // met le pin à HIGH
                *gpio_out_w1ts = 1 << self.pin;
            } else {
                // met le pin à LOW
                *gpio_out_w1tc = 1 << self.pin;
            }
        }
    }

    // lit l'état du pin
    fn read(&self) -> bool {
        let gpio_in = unsafe { (GPIO_BASE + GPIO_IN_OFFSET) as *const u32 };

        unsafe {
            // lit l'état du pin
            (*gpio_in & (1 << self.pin)) != 0
        }
    }
}
