.PHONY: server client

key_pair:
	cargo run --bin cli -- gen_key_pair --output key_pair.p8

salt_secret:
	cargo run --bin cli -- gen_salt_secret --output salt_secret

state: key_pair salt_secret

update:
	cargo update

upgrade_dry_run:
	cargo upgrade --workspace --dry-run

upgrade:
	cargo upgrade --workspace

server:
	cargo run --bin server

client:
	cargo run --bin client

fmt:
	cargo fmt

check:
	cargo check

doc:
	cargo doc --open

test:
	cargo test