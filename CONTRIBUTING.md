# WASM-4 Snake Game

## Quick Start for Contributors

### Option 1: Using DevContainer (Recommended)

1. Install [Docker](https://www.docker.com/get-started) and [VS Code](https://code.visualstudio.com/)
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Clone this repo and open in VS Code
4. Click "Reopen in Container" when prompted
5. Run `./build.sh` to build the game

The DevContainer automatically sets up:
- ✅ Rust 1.90.0+ with wasm32 target
- ✅ wasm-opt optimization tools
- ✅ WASM-4 CLI
- ✅ rust-analyzer and VS Code extensions

### Option 2: Manual Setup

If you prefer not to use DevContainers:

1. Install Rust 1.78.0 or higher:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. Install binaryen (for wasm-opt):
   ```bash
   # Ubuntu/Debian
   sudo apt-get install binaryen
   
   # macOS
   brew install binaryen
   
   # Or download from: https://github.com/WebAssembly/binaryen/releases
   ```

3. Install WASM-4 CLI:
   ```bash
   npm install -g wasm4
   ```

4. Build and run:
   ```bash
   ./build.sh
   w4 run cart.wasm
   ```

## Development Workflow

1. **Build**: `./build.sh` - Builds and optimizes the cart
2. **Run**: `w4 run cart.wasm` - Test in WASM-4 runtime
3. **Watch**: `w4 watch` - Auto-reload on changes
4. **Bundle**: `w4 bundle cart.wasm --title "Snake" --html game.html`

## Troubleshooting

### "Cart is above the size limit"
The project is configured to stay under 64KB. If you hit this:
- Check [SIZE_OPTIMIZATION.md](SIZE_OPTIMIZATION.md) for tips
- Run `cargo bloat --release --target wasm32-unknown-unknown` to see what's taking space

### rust-analyzer errors
- Make sure you're using Rust 1.78.0+: `rustc --version`
- Try reloading VS Code window
- The DevContainer handles this automatically

### Build fails with bulk memory errors
- Update your Rust toolchain: `rustup update stable`
- The build script enables these features in wasm-opt

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure `./build.sh` completes successfully
5. Submit a pull request

## License

See LICENSE file for details.
