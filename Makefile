CC ?= gcc
CARGO ?= cargo
CFLAGS += -Wall -Iinclude/
LDFLAGS += -Ltarget/release -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm
OBJ := cboxxy.o

all: cboxxy

cboxxy: $(OBJ) target/release/libboxxy.a
	$(CC) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

target/release/libboxxy.a:
	$(CARGO) build --verbose --release

.PHONY: clean target/release/libboxxy.a
clean:
	rm -rf cboxxy *.o target/
