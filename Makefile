TARGET=x86_64-unknown-linux-musl
all:
	cargo build
	cargo build --release
clean:
	cargo clean
	rm submit*.zip
test:
	cargo test
submit:
	# debug
	cargo build
	cp target/debug/codevs_reborn togatogAI
	zip submit_debug.zip togatogAI run.sh
	# release
	cargo build --release
	cp target/release/codevs_reborn togatogAI
	zip submit_release.zip togatogAI run.sh
