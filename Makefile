SHELL = /bin/bash

ARTIFACT := open-fork.tar.gz
DEV_HTTPS_KEY := key.pem
DEV_HTTPS_CERT := cert.pem
BUILD_FILES := target/release/open-fork

DEV_DATABASE_URL := db.sqlite3
DEV_DATABASE_SEED := db.seeded
DIESEL_DB_ARGS = --migration-dir "./migrations/db" --database-url "$(DEV_DATABASE_URL)"
DIESEL_SEED_ARGS = --migration-dir "./migrations/seed" --database-url "$(DEV_DATABASE_URL)"

.PHONY: build package db dev-db ssl dev new-migration deps

build $(BUILD_FILES):
	cargo build --release

package: $(BUILD_FILES)
	echo "TAR GZ"

db $(DEV_DATABASE_URL):
	rm -f $(DEV_DATABASE_URL)
	rm -f $(DEV_DATABASE_SEED)
	diesel migration run $(DIESEL_DB_ARGS)

db-seed $(DEV_DATABASE_SEED): $(DEV_DATABASE_URL)
	diesel migration run $(DIESEL_SEED_ARGS)
	touch $(DEV_DATABASE_SEED)

ssl $(DEV_HTTPS_KEY): 
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

dev: $(DEV_CERT) $(DEV_DATABASE_SEED)
	cargo watch --watch ./src --exec run

new-migration: $(DEV_DATABASE_URL)
	@[ "$(NAME)" == "" ] && echo "NAME argument required" && exit 1 || true
	diesel migration generate $(DIESEL_DB_ARGS) $(NAME)
	sleep 1
	diesel migration generate $(DIESEL_SEED_ARGS) $(NAME)

deps:
	sudo apt-get install -y libssl-dev libsqlite3-dev
	cargo install diesel_cli --no-default-features --features sqlite
	cargo install cargo-watch
