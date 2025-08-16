# Makefile for download_neat project

.PHONY: help build install uninstall purge clean status logs

# Project variables
PROJECT_NAME = download_neat
BINARY_NAME = download_neat
SERVICE_NAME = download-neat
BINARY_INSTALL_PATH = /usr/local/bin/$(BINARY_NAME)
SERVICE_FILE = /etc/systemd/user/$(SERVICE_NAME).service

# Default target
help:
	@echo "Available targets:"
	@echo "  build     - Build the release version"
	@echo "  install   - Build and install the service"
	@echo "  uninstall - Remove the service and binary"
	@echo "  purge     - Complete removal (uninstall + clean)"
	@echo "  clean     - Clean build artifacts"
	@echo "  status    - Check service status"
	@echo "  logs      - View service logs"
	@echo "  help      - Show this help message"

# Build the release version
build:
	@echo "Building $(PROJECT_NAME)..."
	cargo build --release
	@echo "Build completed!"

# Install the service
install:
	@echo "Installing $(PROJECT_NAME)..."
	@chmod +x install.sh
	./install.sh

# Uninstall the service and binary
uninstall:
	@echo "Uninstalling $(PROJECT_NAME)..."
	@if systemctl --user is-active --quiet $(SERVICE_NAME); then \
		echo "Stopping service..."; \
		systemctl --user stop $(SERVICE_NAME); \
	fi
	@if systemctl --user is-enabled --quiet $(SERVICE_NAME); then \
		echo "Disabling service..."; \
		systemctl --user disable $(SERVICE_NAME); \
	fi
	@echo "Removing systemd service file..."
	@sudo rm -f $(SERVICE_FILE)
	@echo "Removing binary..."
	@sudo rm -f $(BINARY_INSTALL_PATH)
	@echo "Reloading systemd daemon..."
	@systemctl --user daemon-reload
	@echo "Uninstallation completed!"

# Complete removal (uninstall + clean)
purge: uninstall clean
	@echo "Purging $(PROJECT_NAME) completed!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean completed!"

# Check service status
status:
	@echo "Service status:"
	@systemctl --user status $(SERVICE_NAME) --no-pager -l || echo "Service not found or not running"

# View service logs
logs:
	@echo "Service logs (press Ctrl+C to exit):"
	@journalctl --user -u $(SERVICE_NAME) -f || echo "No logs found"

# Quick test run
test-run:
	@echo "Running $(PROJECT_NAME) in foreground (press Ctrl+C to stop)..."
	cargo run
