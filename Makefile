SHELL = /bin/bash

ARTIFACT := open-fork.tar.gz
DEV_HTTPS_KEY := key.pem
DEV_HTTPS_CERT := cert.pem
BUILD_FILES := target/release/open-fork

DEV_DB_FILE := db.sqlite3
DEV_DATABASE_URL := sqlite:$(DEV_DB_FILE)
DEV_DATABASE_SEED := db.seeded

.PHONY: build package db db-seed db-info ssl dev new-migration deps

build $(BUILD_FILES):
	cargo build --release

package: $(BUILD_FILES)
	echo "TAR GZ"

db $(DEV_DB_FILE):
	rm -f $(DEV_DV_FILE)
	rm -f $(DEV_DATABASE_SEED)
	sqlx database create --database-url "$(DEV_DATABASE_URL)" --sqlite-create-db-wal false
	sqlx migrate --source "./migrations/db" run --database-url "$(DEV_DATABASE_URL)"

db-seed $(DEV_DATABASE_SEED): $(DEV_DB_FILE)
	sqlx migrate --source "./migrations/seed" run --database-url "$(DEV_DATABASE_URL)" --ignore-missing
	touch $(DEV_DATABASE_SEED)

db-info:
	sqlx migrate --source "./migrations/db" info --database-url "$(DEV_DATABASE_URL)"
	sqlx migrate --source "./migrations/seed" info --database-url "$(DEV_DATABASE_URL)"

ssl $(DEV_HTTPS_KEY): 
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

dev: $(DEV_CERT) $(DEV_DATABASE_SEED)
	cargo watch --watch ./src --exec run

new-migration:
	@[ "$(NAME)" == "" ] && echo "NAME argument required" && exit 1 || true
	sqlx migrate --source "./migrations/db" add $(NAME)
	sleep 1
	sqlx migrate --source "./migrations/seed" add $(NAME)

deps:
	sudo apt-get install -y libssl-dev libsqlite3-dev
	cargo install sqlx
	cargo install cargo-watch
