PREFIX = arm-none-eabi

RUSTC   = rustc
LD      = $(PREFIX)-gcc
OBJCOPY = $(PREFIX)-objcopy

ARCHFLAGS = -mcpu=cortex-m4 -mthumb -mthumb-interwork
FPFLAGS   = -mfloat-abi=hard -mfpu=fpv4-sp-d16

RUSTFLAGS  = --target thumbv7em-none-eabi -O -g -Z no-landing-pads -L lib
# RUSTFLAGS  = -C lto
RUSTFLAGS += -A dead_code

LDFLAGS  = $(ARCHFLAGS) $(FPFLAGS)
LDFLAGS += --static -nostartfiles -Tstm32f3-discovery.ld -Llib
LDFLAGS += -Wl,--no-warn-mismatch,--gc-sections
LDFLAGS += -Wl,--start-group -lnosys -Wl,--end-group
# LDFLAGS += -flto

RUSTSRC = src/main.rs
OBJS += $(RUSTSRC:.rs=.o)

all: main.hex main.elf

lib/libcore.rlib: support/rust/src/libcore/lib.rs
	$(RUSTC) $(RUSTFLAGS) --out-dir lib $<

%.o: %.rs src/*.rs lib/libcore.rlib
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

%.elf: $(OBJS)
	$(LD) $(ARCHFLAGS) $(LDFLAGS) $(LDLIBS) $(OBJS) -o $@

%.hex: %.elf
	$(OBJCOPY) -O ihex $< $@

upload: main.hex
	openocd -f interface/stlink-v2.cfg -f board/stm32f3discovery.cfg \
	-c "init" -c "reset init" -c "flash write_image erase $<" \
	-c "reset" -c "shutdown"

clean:
	rm main.hex main.elf
