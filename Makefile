VERSION=1.0
NAME=threadbag-makefile

check:
	cargo hack check --no-dev-deps --release

test:
	cargo test --release -- --nocapture 

testweb: 
	cargo test --all-features test_webserver --  --nocapture

fmt: 
	cargo +nightly fmt --all

build: 
	cargo build --release

run_debug:
	TOKIO_CONSOLE_BIND="localhost:6666" RUSTFLAGS="--cfg tokio_unstable" cargo run                     

run_console:
	tokio-console http://localhost:6666

build_unstable:
	RUSTFLAGS="--cfg tokio_unstable" cargo build  --features nightly

install_console:
	cargo install --locked tokio-console

checkdep:
	cargo +nightly udeps

kill_threadbag:
	@ps waux | grep '[t]hreadbag' | awk '{print $$2}' | xargs -r kill -9
