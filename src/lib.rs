// ce fichier permet au main.rs d'accéder aux fonctions des différents modules pour l'USART, le GPIO, le SPI ainsi que l'I2C
// les fonctions et objets importés sont différents selon la cible sélectionnée

#![no_std] // pas de librairie standard


// GPIO
#[cfg(feature = "atmega328p")]
pub mod gpio {
    pub mod gpio_atmega328p;
    pub use gpio_atmega328p::{AtmegaGpio,PinMode,Gpio};

}

#[cfg(feature = "esp32_s3")]
pub mod gpio {
    pub mod gpio_esp32s3;
    pub use gpio_esp32s3::{Esp32Gpio,PinMode,Gpio};
}

// USART
#[cfg(feature = "atmega328p")]
pub mod usart {
    pub mod usart_atmega328p;
    pub use usart_atmega328p::{usart_init as atmega_usart_init, usart_receive as atmega_usart_receive, usart_send as atmega_usart_send};
}

#[cfg(feature = "esp32_s3")]
pub mod usart {
    pub mod usart_esp32s3;
    pub use usart_esp32s3::{usart_init as esp_usart_init, usart_receive as esp_usart_receive, usart_send as esp_usart_send};
}

// SPI
#[cfg(feature = "atmega328p")]
pub mod spi {
    pub mod spi_atmega328p;
    pub use spi_atmega328p::{SpiMode, spi_init as atmega_spi_init, spi_transceive as atmega_spi_transceive, spi_receive as atmega_spi_receive};
}

#[cfg(feature = "esp32_s3")]
pub mod spi {
    pub mod spi_esp32s3;
    pub use spi_esp32s3::{SpiMode, spi_init as esp_spi_init, spi_transceive as esp_spi_transceive, spi_receive as esp_spi_receive};
}

// I2C
#[cfg(feature = "atmega328p")]
pub mod i2c {
    pub mod i2c_atmega328p;
    pub use i2c_atmega328p::AtmegaI2c;
}

#[cfg(feature = "esp32_s3")]
pub mod i2c {
    pub mod i2c_esp32s3;
    pub use i2c_esp32s3::Esp32I2c;
}