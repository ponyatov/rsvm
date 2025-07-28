RTARGET = x86_64-unknown-linux-gnu
# RTARGET = aarch64-unknown-linux-gnu
# RTARGET = i686-pc-windows-gnu

$(RUSTUP) $(CARGO):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup target add $(RTARGET)
	rustup target add wasm32-unknown-unknown
	rustup component add rust-analyzer rustfmt
	cargo  install cargo-watch cargo-binutils
# rustup target add x86_64-unknown-linux-gnu
# rustup target add aarch64-unknown-linux-gnu
# rustup target add i686-pc-windows-gnu
# rustup target add wasm32-unknown-unknown

.PHONY: server
server: $(R)
	cargo watch -x "run --bin $@ --target $(RTARGET) -- $(S)"

.PHONY: main
main: $(R)
	cargo watch -x "run --bin $@ --target $(RTARGET) -gnu -- $(S)"
