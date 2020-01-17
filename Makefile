BIN=fw
OUTDIR=target/thumbv7em-none-eabi/release
HEX=$(OUTDIR)/$(BIN).hex
ELF=$(OUTDIR)/$(BIN)

all:: $(ELF)

.PHONY: $(ELF)
$(ELF):
	cargo build --release

$(HEX): $(ELF)
	arm-none-eabi-objcopy -O ihex $(ELF) $(HEX)

hex: $(HEX)
	@echo Build done

clear:
	@nrfjprog --eraseall

flash:
	@nrfjprog --reset --verify --program $(HEX) --sectoranduicrerase

gdb:
	JLinkGDBServer -device NRF52840_XXAA -if SWD -speed 4000

openocd:
	openocd -f config/openocd/openocd.cfg

openocd_stlink:
	openocd -f config/openocd/openocd_stlinkv2.cfg
