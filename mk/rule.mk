tmp/hello.wat: static/hello.wasm
	wasm2wat $< -o $@

static/%.wasm: src/%.wat $(MK)
	wat2wasm $< -o $@ && wasm-objdump -x $@
