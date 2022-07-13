cat:
	cat Makefile

# Run tests in single thread mode! Several tests modify the clipboard
# and if they run parallely, they get mixed up.
test:
	cargo test -- --test-threads=1

release:
	cargo build --release
	/bin/rm -f ./cpc
	/bin/mv ./target/release/cpc ./cpc
	upx --best --lzma cpc

clean:
	cargo clean
	/bin/rm -f ./cpc
