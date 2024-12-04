PROJECT_NAME := $(shell grep '^name = ' Cargo.toml | sed 's/name = "\(.*\)"/\1/')

# targets
ATMEGA_TARGET := avr-unknown-gnu-atmega328
ESP32_TARGET := xtensa-esp32s3-espidf

# Port (peut devoir être changé)
PORT := /dev/ttyACM0

BAUD := 115200

# features pour les deux targets
FEATURE_ATMEGA := atmega328p
FEATURE_ESP := esp32_s3

FEATURE := $(FEATURE_ATMEGA) # à changer par $(FEATURE_ESP) pour l'ESP32-S3 ou par $(FEATURE_ATMEGA) pour l'ATMEGA

TARGET = $(ATMEGA_TARGET) # à changer par $(ESP32_TARGET) pour l'ESP32-S3 ou par $(ATMEGA_TARGET) pour l'ATMEGA
TARGET_BIN := target/$(TARGET)/release/personnal_hal.bin
MAKE_BIN := esptool --chip ESP32-S3 elf2image target/xtensa-esp32s3-espidf/release/personnal_hal
AVRDUDE_CMD := avrdude -v -c arduino -p m328p -P $(PORT) -b $(BAUD) -U flash:w:target/$(ATMEGA_TARGET)/release/personnal_hal.elf

ESPTOOL_CMD := esptool --chip esp32s3 --port $(PORT) --baud $(BAUD) write_flash 0x1000 target/xtensa-esp32s3-espidf/release/personnal_hal.bin
ESP_ERASE := esptool.py --chip esp32s3 --port $(PORT) erase_flash
UPLOAD_CMD = $(AVRDUDE_CMD) # à changer par $(ESPTOOL_CMD) pour l'ESP32-S3 ou par $(AVRDUDE_CMD) pour l'ATMEGA

ESPCOMP_CMD = cargo +esp build -Z build-std=core --target $(TARGET) --release --features $(FEATURE)
ATMEGACOMP_CMD = cargo +nightly build -Z build-std=core --target $(TARGET) --release --features $(FEATURE)

COMPILE_CMD = $(ATMEGACOMP_CMD) # à changer par $(ESPCOMP_CMD) pour l'ESP32-S3 ou par $(ATMEGACOMP_CMD) pour l'ATMEGA

#to compile then flash on the atmega
buildatmega: compile upload
#to compile then flash on the esp32-s3
buildesp: compile convertbin erase upload
#to convert .elf to .bin for the esp
convertbin: 
	$(MAKE_BIN)
#to compile project with the chosen target
compile: 
	@echo "Building for target: $(TARGET)"
	$(COMPILE_CMD)
#to flash on board
upload: 
	@echo "Uploading to device..."
	$(UPLOAD_CMD)
#to erase previous flash on the esp
erase:
	$(ESP_ERASE)
#to test usart functionnalities for both targets
testusart:
	sudo minicom -D $(PORT) -b 9600

clean:
	cargo clean
	rm -rf target/$(TARGET)/release/$(PROJECT_NAME)*
