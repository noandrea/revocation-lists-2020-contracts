

build:
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

test:
	cargo test -- --nocapture

clean:
	rm -rf target

deploy:
	target/wasm32-unknown-unknown/release/contract.wasm

