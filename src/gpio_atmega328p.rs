use core::ptr;
use core::panic;
use crate::gpio::{Gpio, PinMode};

const DDRD: *mut u8 = 0x2A as *mut u8;
const PORTD: *mut u8 = 0x2B as *mut u8;
const PIND: *const u8 = 0x29 as *const u8;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

pub struct AtmegaGpio {
    pub pin: u8,
}

impl Gpio for AtmegaGpio {
    fn set_mode(&self, mode: PinMode) {
        unsafe {
            match self.pin {
                0..=7 => {
                    if mode == PinMode::Output {
                        ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) | (1 << self.pin));
                    } else {
                        ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) & !(1 << self.pin));
                    }
                }
                8..=13 => {
                    if mode == PinMode::Output {
                        ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << (self.pin - 8)));
                    } else {
                        ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << (self.pin - 8)));
                    }
                }
                _ => panic!("Unsupported pin"),
            }
        }
    }

    fn write(&self, value: bool) {
        unsafe {
            match self.pin {
                0..=7 => {
                    if value {
                        ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) | (1 << self.pin));
                    } else {
                        ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) & !(1 << self.pin));
                    }
                }
                8..=13 => {
                    if value {
                        ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << (self.pin - 8)));
                    } else {
                        ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) & !(1 << (self.pin - 8)));
                    }
                }
                _ => panic!("Unsupported pin"),
            }
        }
    }

    fn read(&self) -> bool {
        unsafe {
            match self.pin {
                0..=7 => (ptr::read_volatile(PIND) & (1 << self.pin)) != 0,
                8..=13 => (ptr::read_volatile(PINB) & (1 << (self.pin - 8))) != 0,
                _ => panic!("Unsupported pin"),
            }
        }
    }
}
