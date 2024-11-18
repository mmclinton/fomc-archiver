MAKEFILE_DIR := $(CURDIR)
DB_DIR := $(HOME)/.local/share/fomc
DB_FILE := $(DB_DIR)/fomc.db
CONFIG_PATH := $(HOME)/.config/fomc
WRITE_JSON := echo '{"api_key": "$(api_key)"}' > $(CONFIG_PATH)/config.json
MKDIR := mkdir -p

all: setup install cron api_key

.PHONY: setup install cron api_key

setup: 
	@echo
	@echo "Creating a database..."
	@$(MKDIR) $(DB_DIR)
	@if [ ! -f $(DB_FILE) ]; then \
		touch $(DB_FILE); \
		echo "SQLite database created at $(DB_FILE)"; \
	else \
	    echo "It looks like an SQLite database already exists at $(DB_FILE)."; \
	    echo "We will use that database instead."; \
	    echo "If you wish to remove the old database, run the following command: rm $(DB_FILE)"; \
	fi
	@echo

install: 
	@echo "Installing..."
	@echo "Building the release binary..."
	@cargo build --release > /dev/null 2>&1
	@echo "Moving the binary to /usr/local/bin..."
	@if [ ! -d "/usr/local/bin" ]; then sudo mkdir -p /usr/local/bin; fi
	@sudo cp target/release/fomc-rss /usr/local/bin/fomc
	@echo "Done!"
	@echo

cron: 
	@echo "Setting up a cron job..."
	@chmod +x cron.sh
	@./cron.sh
	@echo "Done!"
	@echo

api_key: 
	@echo "Adding your API key..."
	@echo "Your api key has been placed at $(CONFIG_PATH)/config.json"
	@$(MKDIR) $(CONFIG_PATH)
	@$(WRITE_JSON)
	@echo "Done!"