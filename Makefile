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


ESPFLASH_CMD := espflash flash -p $(PORT) target/$(ESP32_TARGET)/release/personnal_hal


TARGET = $(ATMEGA_TARGET) # à changer par $(ESP32_TARGET) pour l'ESP32-S3 ou par $(ATMEGA_TARGET) pour l'ATMEGA
TARGET_ELF := target/$(TARGET)/release/personnal_hal.elf

AVRDUDE_CMD := avrdude -v -c arduino -p m328p -P $(PORT) -b $(BAUD) -U flash:w:$(TARGET_ELF)
UPLOAD_CMD = $(AVRDUDE_CMD) # à changer par $(ESPFLASH_CMD) pour l'ESP32-S3 ou par $(AVRDUDE_CMD) pour l'ATMEGA

ESPCOMP_CMD = cargo +esp build -Z build-std=core --target $(TARGET) --release --features $(FEATURE)
ATMEGACOMP_CMD = cargo +nightly build -Z build-std=core --target $(TARGET) --release --features $(FEATURE)

COMPILE_CMD = $(ATMEGACOMP_CMD) # à changer par $(ESPCOMP_CMD) pour l'ESP32-S3 ou par $(ATMEGACOMP_CMD) pour l'ATMEGA

#to compile then flash on the target
build: compile upload
#to compile project with the chosen target
compile: 
	@echo "Building for target: $(TARGET)"
	$(COMPILE_CMD)
#to flash on board
upload: 
	@echo "Uploading to device..."
	$(UPLOAD_CMD)

testusart:
	sudo minicom -D $(PORT) -b 9600

clean:
	cargo clean
	rm -f target/$(TARGET)/release/$(PROJECT_NAME)*
