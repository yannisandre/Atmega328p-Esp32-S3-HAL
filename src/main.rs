#![no_std] // pas de librairie standard pour nos cibles (MCU embarqué)
#![no_main] // pas de fonction main

use core::panic::PanicInfo;

// Import des fonctions pour le GPIO, l'USART, le SPI & l'I2C de l'ATMEGA
#[cfg(feature = "atmega328p")]
use personnal_hal::gpio::{AtmegaGpio,PinMode,Gpio};
#[cfg(feature = "atmega328p")]
use personnal_hal::usart::{atmega_usart_init,atmega_usart_send,atmega_usart_receive};
#[cfg(feature = "atmega328p")]
use personnal_hal::spi::{SpiMode,atmega_spi_init,atmega_spi_transceive,atmega_spi_receive};
#[cfg(feature = "atmega328p")]
use personnal_hal::i2c::{AtmegaI2c};

// Import des fonctions pour le GPIO, l'USART, le SPI & l'I2C de l'ESP32-S3
#[cfg(feature = "esp32_s3")]
use personnal_hal::gpio::{Esp32Gpio,PinMode,Gpio};
#[cfg(feature = "esp32_s3")]
use personnal_hal::usart::{esp_usart_init,esp_usart_send,esp_usart_receive};
#[cfg(feature = "esp32_s3")]
use personnal_hal::spi::{SpiMode,esp_spi_init,esp_spi_receive,esp_spi_transceive};
#[cfg(feature = "esp32_s3")]
use personnal_hal::i2c::{Esp32I2c};


// on definit le débit de communication pour l'USART (9600 bauds)
const BAUD_RATE: u32 = 9600;

#[no_mangle]
pub extern "C" fn main() -> ! {

    // Tests de l'USART + GPIO + SPI + I2C pour l'ATMEGA
    #[cfg(feature = "atmega328p")]
    {
             
        
        /*

        // Test ATMEGA USART

        atmega_usart_init(9600);  // Initialisation de la communication USART  avec 9600 bauds

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
        
        

        
        // TEST ATMEGA SPI
        
        /*
        
        // Test SPI en mode Master

        atmega_usart_init(9600);

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

        atmega_usart_init(9600);

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

        // TEST ATMEGA I2C

        /*

        atmega_usart_init(9600);
        // debug message
        let startup_message = b"I2C TEST START\r\n";
        for &byte in startup_message {
            atmega_usart_send(byte);
        }

        // I2C initialization at 100 kHz
        AtmegaI2c::init(100_000, 16_000_000); // I2C frequency : 100 kHz, CPU frequency : 16 MHz

        // I2C test : fictive slave peripheral
        let slave_address = 0x50; // Example : EEPROM (classic register)

        // START condition
        atmega_usart_send(b'S'); // Log START condition sent
        AtmegaI2c::start();

        // sending of the slave address in write mode
        AtmegaI2c::write_byte((slave_address << 1) | 0); //  address & writing bit

        // sending datas
        let test_data: [u8; 3] = [0xDE, 0xAD, 0xBE];
        for &byte in &test_data {
            AtmegaI2c::write_byte(byte);
        }

        // STOP condition
        AtmegaI2c::stop();
        atmega_usart_send(b'E'); // log END

        let end_message = b"I2C TEST END\r\n"; // fin du test
        for &byte in end_message {
            atmega_usart_send(byte);
        }
        
        */
        
    }

    // Tests de l'USART + GPIO + SPI + I2C pour l'ESP32-S3
    #[cfg(feature = "esp32_s3")]
    {
        
        // Test ESP32-S3 GPIO

        /*

        let gpio = Esp32Gpio { pin: 5};
        gpio.set_mode(PinMode::Output);  
        gpio.write(true);  

        */
        

        // Test ESP32-S3 USART

        /*
        
        esp_usart_init(BAUD_RATE); // Configure l'USART à 9600 baud
        let message = b"HELLO"; // envoie du message "HELLO" lettre par lettre
        for &byte in message.iter() {
            esp_usart_send(byte);
        let message: u8 = esp_usart_receive();
        }

        */

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
        

        // TEST ESP32-S3 I2C
        
        // Mode Master

        /*

        Esp32I2c::init(true); // true pour le mode master

       
        Esp32I2c::start(); // envoie une condition START

        let slave_address = 0x50; // on envoie l'adresse esclave en écriture
        Esp32I2c::write_byte((slave_address << 1) | 0);

        // on envoie des données
        let test_data: [u8; 3] = [0xDE, 0xAD, 0xBE];
        for &byte in &test_data {
            Esp32I2c::write_byte(byte);
        }

        // on envoie une condition STOP
        Esp32I2c::stop();

        */

        /*
        
        // Mode Slave

        Esp32I2c::init(false); // false pour le mode esclave

        // on recoie l'adresse esclave
        let received_address = Esp32I2c::read_byte(true); // on envoie l'ACK

        let received_data = Esp32I2c::read_byte(false); // on envoie le NACK après le dernier envoie de byte
        
        */


    }

    loop {} // Boucle infinie pour empêcher la sortie du programme
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
