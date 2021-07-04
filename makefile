CPU			:= atmega2560
PROG		:= wiring
BAUD		:= 115200
PORT		:= /dev/ttyUSB0

DIR			:= target/avr-atmega2560/release

HEX			:= AtRife.hex
ELF			:= AtRife.elf

PACMAN		:= cargo
CLEAN		:= clean
COMPILER	:= build
COMP_FL		:= --release

AVR_CMP		:= avr-objcopy
AVR_FL		:= -O ihex

UPLOADER	:= avrdude
UPL_FL		:= -v -p $(CPU) -c $(PROG) -P $(PORT) -b $(BAUD) -F -D -U flash:w

all: clean compile link load

clean:
	$(PACMAN) $(CLEAN)
	-rm $(HEX)

compile: clean
	$(PACMAN) $(COMPILER) $(COMP_FL)

link:
	$(AVR_CMP) $(AVR_FL) $(DIR)/$(ELF) $(HEX)

load:
	$(UPLOADER) $(UPL_FL):$(HEX)
