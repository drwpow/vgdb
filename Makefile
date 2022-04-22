.PHONY: all db

all: db

db:
		@echo "Creating database…"
		rm -rf db/vgdb.sqlite
		cat db/vgdb_create.sql | sqlite3 db/vgdb.sqlite
		@echo "✔ Done"