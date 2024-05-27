VERSION=1.0
NAME=threadbag-makefile

check:
	cargo hack check --no-dev-deps --release

test:
	cargo test -- --nocapture

fmt: 
	cargo +nightly fmt --all

build: 
	cargo build --release

checkdep:
	cargo +nightly udeps

kill_threadbag:
	@ps waux | grep '[t]hreadbag' | awk '{print $$2}' | xargs -r kill -9