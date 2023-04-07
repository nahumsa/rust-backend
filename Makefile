make setup:
	cargo install cargo-watch

run_cargo_watch:
	cargo watch -x check -x test -x run

fmt:
	cargo clippy
	cargo fmt

check_style:
	cargo clippy -- -D warnings
	cargo fmt -- --check