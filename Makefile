CC ?= gcc
CARGO ?= cargo
CFLAGS += -Wall -Iinclude/
LDFLAGS += -Ltarget/release -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm
OBJ := cboxxy.o

all: cboxxy sc/ohai.txt sc/ret.txt

cboxxy: $(OBJ) target/release/libboxxy.a
	$(CC) -o $@ $^ $(LDFLAGS)

# Shell code

sc/%.txt: sc/%.bin
	cargo run --example objdump $^ > $@

sc/%.bin: sc/%.o
	ld -s -m elf_x86_64 -o $@ $^

sc/%.o: sc/%.asm
	nasm -f elf64 -o $@ $^

# C code

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

target/release/libboxxy.a:
	$(CARGO) build --verbose --release

.PHONY: clean target/release/libboxxy.a
clean:
	rm -rf cboxxy *.o sc/*.txt sc/*.o sc/*.bin target/
