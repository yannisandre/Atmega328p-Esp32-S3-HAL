# Atmega328p-Esp8266-HAL
Hardware abstraction layer for the Atmega328p &amp; ESP32-S3 using Rust

Current features implemented : GPIO, USART, SPI & I2C for Atmega328p & ESP32-S3

To run the project :

-git clone this repo

-cd <repo_name>

-to compile : make compile

-to upload on a board : make upload

-to compile and upload : make buildatmega/buildesp

You may have to change the PORT depending on which port your board is plugged.
If you want to change target you'll have to switch some variables in the MakeFile in order to compile and upload (read the comments in the Makefile)

Yannis Andre OCC1

[CORRECTION SPI] (Don't hesitate to remove this part)
Good project.
You could eventually organize your files with folder modules (with all the spi features files in one folder for example).
