build-release:
	cargo build --workspace --release

format:
	cargo fmt

lint:
	cargo clippy

env-up:
	docker compose up -d

env-down:
	docker compose down

test:
	cargo test