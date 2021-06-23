all: build
.PHONY: all

target/x86_64-unknown-linux-gnu/release/bootstrap: src/main.rs Cargo.toml Cargo.lock
	cargo build --release --target x86_64-unknown-linux-gnu

build: target/x86_64-unknown-linux-gnu/release/bootstrap template.yaml
	sam build
.PHONY: build

deploy: build
	sam deploy
.PHONY: deploy

build-FizzBuzzFunction:
	cp ./target/x86_64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
.PHONY: build-FizzBuzzFunction

clean:
	rm -rf .aws-sam target