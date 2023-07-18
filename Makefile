all: run-api run-ui

run-api:
	cargo run --manifest-path api/Cargo.toml

run-ui:
	cd ui && flutter run
