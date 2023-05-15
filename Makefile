make setup:
	cargo install cargo-watch
	cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
	cargo install cargo-udeps
	cargo install bunyan

run_cargo_watch:
	cargo watch -x check -x test -x run

fmt:
	cargo clippy
	cargo fmt

check_style:
	cargo clippy -- -D warnings
	cargo fmt -- --check

create_subscription_migration:
	sqlx migrate add create_subscriptions_table
