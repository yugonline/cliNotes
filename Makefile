.PHONY: build init-db start

# Default target
all: build init-db start

# Build the Rust project
build:
	cargo build

# Initialize the SQLite database
init-db:
	sqlite3 clidblocal.db < src/init.sql

# Run the built binary
start:
	target/debug/cliNotes

clean:
	cargo clean
