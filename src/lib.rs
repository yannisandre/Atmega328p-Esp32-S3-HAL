// ce fichier permet au main.rs d'accéder aux fonctions des différents modules pour l'USART + GPIO
// les fonctions et objets importés sont différents selon la cible sélectionnée

#![no_std] // pas de librairie standard


#[cfg(feature = "atmega328p")]
mod gpio_atmega328p;
#[cfg(feature = "atmega328p")]
pub use gpio_atmega328p::AtmegaGpio;

#[cfg(feature = "esp32_s3")]
pub mod gpio_esp32s3;
#[cfg(feature = "esp32_s3")]
pub use gpio_esp32s3::Esp32Gpio;

pub mod gpio;

#[cfg(feature = "atmega328p")]
mod usart_atmega328p;
#[cfg(feature = "atmega328p")]
pub use usart_atmega328p::{usart_init as atmega_usart_init, usart_receive as atmega_usart_receive, usart_send as atmega_usart_send};

#[cfg(feature = "esp32_s3")]
mod usart_esp32s3;
#[cfg(feature = "esp32_s3")]
pub use usart_esp32s3::{usart_init as esp32_usart_init, usart_receive as esp32_usart_receive, usart_send as esp32_usart_send};
