.PHONY: wasm
wasm: static/$(APP).wasm tmp/hello.wat

.PHONY: all run
all: $(BIN)/$(BINFILE) $(S)
	cargo build
run: $(BIN)/$(BINFILE) $(S)
	cargo run -- $(S)
