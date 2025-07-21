.PHONY: all run
all: $(BIN)/$(BINFILE) $(S)
	cargo build
run: $(BIN)/$(BINFILE) $(S)
	cargo run -- $(S)
