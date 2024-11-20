#![no_std] // pas de librairie standard pour nos cibles (MCU embarqué)
#![no_main] // pas de fonction main

use core::panic::PanicInfo;
use personnal_hal::gpio::{Gpio, PinMode}; // import des structures communes aux deux targets pour l'implémentation du GPIO

// Import des fonctions pour le GPIO et l'USART de l'ATMEGA
#[cfg(feature = "atmega328p")]
use personnal_hal::{AtmegaGpio, atmega_usart_init, atmega_usart_send, atmega_usart_receive};

// Import des fonctions pour le GPIO et l'USART de l'ESP32-S3
#[cfg(feature = "esp32_s3")]
use personnal_hal::{Esp32Gpio, esp32_usart_init, esp32_usart_send, esp32_usart_receive};

// on definit le débit de communication pour l'USART (9600 bauds)
const BAUD_RATE: u32 = 9600;

#[no_mangle]
pub extern "C" fn main() -> ! {

    // Tests de l'USART + GPIO pour l'ATMEGA
    #[cfg(feature = "atmega328p")]
    {
        /* // Test ATMEGA USART
        // Initialisation de la communication USART  avec 9600 bauds
        atmega_usart_init(9600);

        let message = b"HELLO";
        for &byte in message.iter() {
            atmega_usart_send(byte); // envoie d'un octet à la fois
        } 
        let message: u8 = atmega_usart_receive();
        */

        /* // Test ATMEGA GPIO
        let gpio = AtmegaGpio { pin: 5 };
        gpio.set_mode(PinMode::Output);  // Configure le pin 5 comme sortie
        gpio.write(true);                // Met le pin 5 à high
        */
    }

    // Tests de l'USART + GPIO pour l'ESP32-S3
    #[cfg(feature = "esp32_s3")]
    {
        /* // Test ESP32-S3 GPIO
        let gpio = Esp32Gpio { pin: 4 };
        gpio.set_mode(PinMode::Output);  
        gpio.write(true);  
        */        

        /* // Test ESP32-S3 USART
        esp32_usart_init(BAUD_RATE); // Configure l'USART à 9600 baud
        let message = b"HELLO"; // envoie du message "HELLO" lettre par lettre
        for &byte in message.iter() {
            esp32_usart_send(byte);
        }*/
    }

    loop {} // Boucle infinie pour empêcher la sortie du programme
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
