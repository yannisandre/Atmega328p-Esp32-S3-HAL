PROJECT_NAME := $(shell grep '^name = ' Cargo.toml | sed 's/name = "\(.*\)"/\1/')
TARGET := avr-unknown-gnu-atmega328
TARGET_ELF := $(wildcard target/$(TARGET)/release/deps/$(PROJECT_NAME)*.elf)

PORT := /dev/ttyACM0
BAUD := 115200

build: compile upload

compile: 
	cargo +nightly build -Z build-std=core --target $(TARGET) --release

upload: 
	avrdude -c arduino -p m328p -P $(PORT) -b $(BAUD) -U flash:w:$(TARGET_ELF)

# this makefile allows the user to automatically compile the project and flash it on the board (make build)
# this is made for linux OS and may be requiring user to change file path to fit with his own filesystem
# change TARGET with your own needs and the PORT with the one your device uses