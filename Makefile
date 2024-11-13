MAKEFILE_DIR := $(CURDIR)
DB_DIR := $(HOME)/.local/share/fomc
DB_FILE := $(DB_DIR)/fomc.db

all: setup install cron

.PHONY: setup install cron

setup: 
	@echo ""
	@echo "Creating a database..."
	@mkdir -p $(DB_DIR)
	@if [ ! -f $(DB_FILE) ]; then \
		touch $(DB_FILE); \
		echo "SQLite database created at $(DB_FILE)"; \
	else \
	    echo "It looks like an SQLite database already exists at $(DB_FILE)."; \
	    echo "We will use that database instead."; \
	    echo "If you wish to remove the old database, run the following command: rm $(DB_FILE)"; \
	fi
	@echo ""

install: 
	@echo "Installing..."
	@echo "Done!"
	@echo ""

cron: 
	@echo "Setting up a cron job..."
	@chmod +x cron.sh
	@./cron.sh
	@echo "Done!"
	@echo ""
