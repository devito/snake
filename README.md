# hello-word

A Snake game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## Development Environment

This project uses VS Code DevContainers to ensure a consistent development environment.

### Prerequisites

- [Docker](https://www.docker.com/get-started)
- [VS Code](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Getting Started

1. Clone this repository
2. Open the folder in VS Code
3. When prompted, click "Reopen in Container" (or use Command Palette: "Dev Containers: Reopen in Container")
4. Wait for the container to build and setup to complete
5. You're ready to develop!

The DevContainer includes:
- Rust 1.90.0+ (stable)
- `wasm32-unknown-unknown` target
- Binaryen tools (wasm-opt)
- WASM-4 CLI
- rust-analyzer and other useful VS Code extensions

## Building

Build the optimized cart by running:

```shell
./build.sh
```

This will:
1. Build the release target
2. Optimize with wasm-opt
3. Verify the cart is under 64KB
4. Output `cart.wasm`

Or manually build with:

```shell
cargo build --release --target wasm32-unknown-unknown
```

Then run it with:

```shell
w4 run cart.wasm
```

## Bundling

Create a standalone HTML file:

```shell
w4 bundle cart.wasm --title "Snake Game" --html snake-game.html
```

## Size Optimization

This project stays well under WASM-4's 64KB size limit through:
- Removing heavy dependencies (`lazy_static`, `fastrand`)
- Aggressive compiler optimizations (`opt-level = "z"`, LTO, etc.)
- Post-build optimization with `wasm-opt`

See [SIZE_OPTIMIZATION.md](SIZE_OPTIMIZATION.md) for details.

Current cart size: **~8KB** (87% under limit!)

## Project Structure

```
.
├── .devcontainer/          # DevContainer configuration
│   ├── devcontainer.json   # Container setup
│   └── setup.sh           # Environment setup script
├── src/
│   ├── lib.rs             # Main entry point
│   ├── game.rs            # Game logic
│   ├── snake.rs           # Snake implementation
│   ├── palette.rs         # Color palette
│   ├── wasm4.rs           # WASM-4 API bindings
│   └── alloc.rs           # Memory allocator
├── build.sh               # Build and optimization script
├── rust-toolchain.toml    # Rust version specification
└── Cargo.toml             # Project dependencies

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
