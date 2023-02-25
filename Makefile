build:
	@cargo build

clean:
	@cargo clean
	docker stop $$(docker ps -aq) || true
	docker rm $$(docker ps -aq) || true
	docker rmi hworld || true
	docker rmi $$(docker images -f "dangling=true" -q ) || true

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run

alpine:
	docker build -t rust-alpine -f ./alpine/Dockerfile2 .

m1:
	cargo build
	cross build --target x86_64-unknown-linux-musl --release

.PHONY: build test docs style-check lint alpine m1, arm
