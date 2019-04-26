TARGET=x86_64-unknown-linux-musl
NAME=togatog_ai
all:
	cargo build
	cargo build --release
clean:
	cargo clean
	rm submit*.zip
test:
	cargo test
submit:
	cargo build --release --target=x86_64-unknown-linux-musl
	zip -j submit.zip run.sh target/x86_64-unknown-linux-musl/release/$(NAME)
