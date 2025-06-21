.PHONY: build init-db start

# Default target
all: build init-db start

# Build the Rust project
build:
	cargo build

# Initialize the SQLite database
init-db:
	sqlite3 clidblocal.db < sql/init.sql

# Run the built binary
start:
	target/debug/cli_notes

clean:
	cargo clean
