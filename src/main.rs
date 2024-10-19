#![no_std]
#![no_main]

use core::panic::PanicInfo;
use personnal_hal::Gpio; // we import the Gpio structure from the lib.rs (hal implementation) file
use personnal_hal::PinMode; // then we import the PinMode type from lib.rs

// Example of using the HAL to configure and manipulate GPIO pins
#[no_mangle]
pub extern "C" fn main() -> ! {
    let pin = Gpio { pin: 5}; // Create a new GPIO pin instance for pin 5

    pin.set_mode(PinMode::Output); // Set pin 5 as an output
    pin.write(true); // Set pin 5 state at high

    let state = pin.read(); // Read the state of pin 5
    

    loop {} // Infinite loop to keep the program running
}

// A basic panic handler required for no_std programs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
