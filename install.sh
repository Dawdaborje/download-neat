#!/bin/bash

# Install script for download_neat
# This script compiles the Rust project and sets up a systemd service

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   print_error "This script should not be run as root"
   exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

print_status "Starting installation of download_neat..."

# Get the project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_NAME="download_neat"
BINARY_NAME="download_neat"
SERVICE_NAME="download-neat"
SERVICE_FILE="/etc/systemd/user/${SERVICE_NAME}.service"
BINARY_INSTALL_PATH="/usr/local/bin/${BINARY_NAME}"

print_status "Project directory: $PROJECT_DIR"

# Change to project directory
cd "$PROJECT_DIR"

print_status "Building release version..."
cargo build --release

if [ $? -ne 0 ]; then
    print_error "Build failed!"
    exit 1
fi

print_status "Build successful!"

# Check if binary was created
BINARY_PATH="target/release/${BINARY_NAME}"
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binary not found at $BINARY_PATH"
    exit 1
fi

print_status "Installing binary to system..."
# Use sudo to copy binary to system directory
sudo cp "$BINARY_PATH" "$BINARY_INSTALL_PATH"
sudo chmod +x "$BINARY_INSTALL_PATH"

print_status "Creating systemd user service..."

# Create systemd service file
sudo tee "$SERVICE_FILE" > /dev/null << EOF
[Unit]
Description=Download Neat - File organization service
After=network.target

[Service]
Type=simple
ExecStart=$BINARY_INSTALL_PATH
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
EOF

print_status "Reloading systemd daemon..."
systemctl --user daemon-reload

print_status "Enabling service to start on boot..."
systemctl --user enable "$SERVICE_NAME"

print_status "Starting service..."
systemctl --user start "$SERVICE_NAME"

# Check service status
if systemctl --user is-active --quiet "$SERVICE_NAME"; then
    print_status "Service is running successfully!"
else
    print_warning "Service failed to start. Checking status:"
    systemctl --user status "$SERVICE_NAME" --no-pager -l
    print_warning "You can check the logs with: journalctl --user -u $SERVICE_NAME"
fi

print_status "Installation completed!"
echo ""
echo "Service: $SERVICE_NAME"
echo "Binary: $BINARY_INSTALL_PATH"
echo "Service file: $SERVICE_FILE"
echo ""
echo "Useful commands:"
echo "  Check status: systemctl --user status $SERVICE_NAME"
echo "  View logs: journalctl --user -u $SERVICE_NAME -f"
echo "  Stop service: systemctl --user stop $SERVICE_NAME"
echo "  Restart service: systemctl --user restart $SERVICE_NAME"
echo "  Disable service: systemctl --user disable $SERVICE_NAME"
echo ""
echo "To uninstall, run: make purge"
