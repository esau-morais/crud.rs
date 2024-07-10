install:
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-rt
	cargo add listenfd
	cargo add r2d2
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add diesel --features "postgres r2d2 uuid chrono"
	cargo add diesel_migrations

build: 
	cargo build

run:
	cargo run
