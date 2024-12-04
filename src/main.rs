#![no_std] // pas de librairie standard pour nos cibles (MCU embarqué)
#![no_main] // pas de fonction main

use core::panic::PanicInfo;
use personnal_hal::gpio::{Gpio, PinMode}; // import des structures communes aux deux targets pour l'implémentation du GPIO

// Import des fonctions pour le GPIO et l'USART de l'ATMEGA
#[cfg(feature = "atmega328p")]
use personnal_hal::{AtmegaGpio, atmega_usart_init, atmega_usart_send, atmega_usart_receive, SpiMode, atmega_spi_init, atmega_spi_transceive, atmega_spi_receive};

// Import des fonctions pour le GPIO et l'USART de l'ESP32-S3
#[cfg(feature = "esp32_s3")]
use personnal_hal::{Esp32Gpio, esp_usart_init, esp_usart_send, esp_usart_receive, SpiMode, esp_spi_init, esp_spi_receive, esp_spi_transceive};

// on definit le débit de communication pour l'USART (9600 bauds)
const BAUD_RATE: u32 = 9600;

#[no_mangle]
pub extern "C" fn main() -> ! {

    // Tests de l'USART + GPIO + SPI pour l'ATMEGA
    #[cfg(feature = "atmega328p")]
    {
        // Test ATMEGA USART
        // Initialisation de la communication USART  avec 9600 bauds
        
        /*
        atmega_usart_init(9600);

        let message = b"HELLO";
        for &byte in message.iter() {
            atmega_usart_send(byte); // envoie d'un octet à la fois
        } 
        let message: u8 = atmega_usart_receive();
        */
        
        
         // Test ATMEGA GPIO
        let gpio = AtmegaGpio { pin: 5 };
        gpio.set_mode(PinMode::Output);  // Configure le pin 5 comme sortie
        gpio.write(true);                // Met le pin 5 à high
        

        /*
        // TEST ATMEGA SPI
        
        // Test SPI en mode Master
        atmega_usart_send(b'S');
        atmega_usart_send(b'P');
        atmega_usart_send(b'I');
        atmega_usart_send(b' ');
        atmega_usart_send(b'M');
        atmega_usart_send(b'A');
        atmega_usart_send(b'S');
        atmega_usart_send(b'T');
        atmega_usart_send(b'E');
        atmega_usart_send(b'R');
        atmega_usart_send(b'\r');
        atmega_usart_send(b'\n');

        atmega_spi_init(SpiMode::Master);
        let received_master = atmega_spi_transceive(0x55); // Envoi d'un byte (0x55)
        atmega_usart_send(received_master); // Affiche le byte reçu en master
        */
        

        /*
        // Test SPI en mode Slave
        atmega_usart_send(b'S');
        atmega_usart_send(b'P');
        atmega_usart_send(b'I');
        atmega_usart_send(b' ');
        atmega_usart_send(b'S');
        atmega_usart_send(b'L');
        atmega_usart_send(b'A');
        atmega_usart_send(b'V');
        atmega_usart_send(b'E');
        atmega_usart_send(b'\r');
        atmega_usart_send(b'\n');

        atmega_spi_init(SpiMode::Slave);
        let received_slave = atmega_spi_receive(); // Attend un byte envoyé par le master
        atmega_usart_send(received_slave); // Affiche le byte reçu en slave
        */
    }

    // Tests de l'USART + GPIO pour l'ESP32-S3
    #[cfg(feature = "esp32_s3")]
    {
        /*
        // Test ESP32-S3 GPIO
        let gpio = Esp32Gpio { pin: 5};
        gpio.set_mode(PinMode::Output);  
        gpio.write(true);  
        */

        // Test ESP32-S3 USART
        /*
        esp32_usart_init(BAUD_RATE); // Configure l'USART à 9600 baud
        let message = b"HELLO"; // envoie du message "HELLO" lettre par lettre
        for &byte in message.iter() {
            esp32_usart_send(byte);
        let message: u8 = esp32_usart_receive();
        }*/

        // TEST SPI ESP32-S3

        /*
        // Test SPI en mode Master
        esp_usart_send(b'S');
        esp_usart_send(b'P');
        esp_usart_send(b'I');
        esp_usart_send(b' ');
        esp_usart_send(b'M');
        esp_usart_send(b'A');
        esp_usart_send(b'S');
        esp_usart_send(b'T');
        esp_usart_send(b'E');
        esp_usart_send(b'R');
        esp_usart_send(b'\r');
        esp_usart_send(b'\n');

        esp_spi_init(SpiMode::Master);
        let received_master = esp_spi_transceive(0x55); // Envoi d'un byte (0x55)
        esp_usart_send(received_master); // Affiche le byte reçu en master
        */
        

        /*
        // Test SPI en mode Slave
        esp_usart_send(b'S');
        esp_usart_send(b'P');
        esp_usart_send(b'I');
        esp_usart_send(b' ');
        esp_usart_send(b'S');
        esp_usart_send(b'L');
        esp_usart_send(b'A');
        esp_usart_send(b'V');
        esp_usart_send(b'E');
        esp_usart_send(b'\r');
        esp_usart_send(b'\n');

        esp_spi_init(SpiMode::Slave);
        let received_slave = esp_spi_receive(); // Attend un byte envoyé par le master
        esp_usart_send(received_slave); // Affiche le byte reçu en slave
        */

    }

    loop {} // Boucle infinie pour empêcher la sortie du programme
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
