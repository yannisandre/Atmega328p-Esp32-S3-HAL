#![no_std]  // our HAL wont use the std crate (no need)
#![no_main] // no  main function

use core::ptr;

// this file contains the whole HAL implementation


// the following constants are for the Atmega328p registers

// for the pins from 0 to 7 include
const DDRD: *mut u8 = 0x2A as *mut u8; // The port D Data Direction Register (used to set the mode ouput or input to a pin)
const PORTD: *mut u8 = 0x2B as *mut u8; // The port D Data Register (used to set the state HIG or LOW to a pin)
const PIND: *const u8 = 0x29 as *const u8; // The Pin Input Register D (used to read the state of a specific pin)

// for the pins from 8 to 13 include
const DDRB: *mut u8 = 0x24 as *mut u8; // Same thing but for the B register
const PORTB: *mut u8 = 0x25 as *mut u8; 
const PINB: *const u8 = 0x23 as *const u8; 


#[derive(PartialEq)] // allows rust to implement the == and != methods for the new type PinMode
pub enum PinMode {
    Input,
    Output, // the pins can be either an input or an output
}

// the GPIO pins will be represented by a structure with a attribute (pin number) and several methods to mimic the pinMode() and digitalWrite() methods
pub struct Gpio {
    pub pin: u8, // The digital pin number (0-7 for PORTD and 8-13 for PORTB)
}

impl Gpio {
    // function to configure the pin as input or output mimicking the pinMode() function
    pub fn set_mode(&self, mode: PinMode) {
        unsafe {
            match self.pin {
                0..=7 => {
                    // Pins 0 to 7 belong to PORTD
                    if mode == PinMode::Output {
                        ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) | (1 << self.pin));  // applies a mask to set to 1 the bit of DDRD register corresponding to the pin chosen
                    } else {
                        ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) & !(1 << self.pin)); // similarly it applies a mask to set to 0 the right bit of DDRD 
                    }
                }
                8..=13 => {
                    // Pins 8 to 13 belong to PORTB
                    if mode == PinMode::Output {
                        ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << (self.pin - 8))); // same thing as above but for the DDRB register
                    } else {
                        ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << (self.pin - 8)));
                    }
                }
                _ => panic!("Unsupported pin"), // not a digital pin (0-13), exception raised
            }
        }
    }

    // function to set the state of a pin (HIGH or LOW) mimicking the digitalWrite() function
    pub fn write(&self, value: bool) {
        unsafe {
            match self.pin {
                0..=7 => {
                    // Pins 0 to 7 are on PORTD
                    if value {
                        ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) | (1 << self.pin)); // applies a mask to set to 1 the bit of PORTD register corresponding to the pin chosen
                    } else {
                        ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) & !(1 << self.pin)); // similarly it applies a mask to set to 0 the right bit of PORTD
                    }
                }
                8..=13 => {
                    // Pins 8 to 13 are on PORTB
                    if value {
                        ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << (self.pin - 8))); // same thing as above but for the PORTB register
                    } else {
                        ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) & !(1 << (self.pin - 8)));
                    }
                }
                _ => panic!("Unsupported pin"), // not a digital pin (0-13), exception raised
            }
        }
    }

    // function to read the state of a specific pin
    pub fn read(&self) -> bool {
        unsafe {
            match self.pin {
                0..=7 => { // 
                    let pin = ptr::read_volatile(PIND);
                    (pin & (1 << self.pin)) != 0 // returns true if pin is high
                }
                8..=13 => {
                    let pin = ptr::read_volatile(PINB);
                    (pin & (1 << (self.pin - 8))) != 0 // returns true if pin is high
                }
                _ => panic!("Unsupported pin"), // not a digital pin (0-13), exception raised
            }
        }
    }
}
