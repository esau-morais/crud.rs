MIGRATION_NAME=

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
