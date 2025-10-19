#!/bin/bash
set -e

echo "🔧 Setting up WASM-4 development environment..."

# Install Rust toolchain and targets
echo "📦 Installing Rust stable toolchain..."
# Ensure rustup has proper permissions
sudo chown -R vscode:vscode /usr/local/rustup 2>/dev/null || true
# Reinstall stable toolchain to ensure all components work properly
rustup toolchain uninstall stable 2>/dev/null || true
rustup toolchain install stable
rustup default stable
rustup target add wasm32-unknown-unknown

# Install wasm-opt (part of binaryen)
echo "📦 Installing binaryen (wasm-opt)..."
sudo apt-get update
sudo apt-get install -y binaryen

# Install WASM-4 CLI
echo "📦 Installing WASM-4 CLI..."
npm install -g wasm4

# Verify installations
echo ""
echo "✅ Setup complete!"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "wasm-opt version: $(wasm-opt --version)"
echo "w4 version: $(w4 --version)"
echo ""
echo "🎮 Ready to build WASM-4 games!"
echo "Run './build.sh' to build your game"
