MIGRATION_NAME=

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
	cargo add dotenvy
	cargo add uuid --features "serde v4"
	cargo add diesel --features "postgres r2d2 uuid chrono"
	cargo add diesel_migrations

setup_diesel:
	diesel setup

generate_new_migration:
	diesel migration generate $(MIGRATION_NAME)

generate_diff_migration:
	diesel migration generate --diff-schema $(MIGRATION_NAME)

run_migrations:
	diesel migration run

build: 
	cargo build

watch:
	cargo watch -x run
