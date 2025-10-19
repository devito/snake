# WASM-4 Cart Size Optimization

## Problem
The cart exceeded WASM-4's 64KB (65535 bytes) size limit at 142KB.

## Solutions Applied

### 1. Removed Heavy Dependencies
- **Removed `lazy_static`**: Replaced with `static mut` and `UnsafeCell` for simpler global state
- **Removed `fastrand`**: Implemented a simple LCG (Linear Congruential Generator) RNG instead

### 2. Optimized Cargo Profile
Added these settings to `[profile.release]` in `Cargo.toml`:
```toml
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
strip = true          # Strip debug symbols (requires Rust 1.78+)
codegen-units = 1     # Single codegen unit for better optimization
panic = "abort"       # Smaller panic handler
```

### 3. Post-Build Optimization with wasm-opt
Used `wasm-opt` from the Binaryen toolkit:
```bash
wasm-opt -Oz --strip-debug --strip-producers \
    --enable-bulk-memory --enable-sign-ext \
    cart.wasm -o cart-opt.wasm
```

Note: `--enable-bulk-memory` and `--enable-sign-ext` are required for Rust 1.78+ which uses these WebAssembly features.

## Results
- **Before**: 142 KB (145,408 bytes)
- **After**: 8.2 KB (8,211 bytes)
- **Reduction**: 94% smaller!

## Toolchain Requirements
- **Rust**: 1.78.0 or higher (for `strip = true` and rust-analyzer support)
- **Toolchain**: Rust 1.90.0 (current)

## Build Process
Simply run:
```bash
./build.sh
```

This will:
1. Build the release target
2. Optimize with wasm-opt
3. Check size against the limit
4. Copy the optimized cart to `cart.wasm`

## Additional Tips for Future Size Reduction

### If you need to reduce size further:
1. **Disable the allocator** if you don't use dynamic allocations:
   ```bash
   cargo build --release --target wasm32-unknown-unknown --no-default-features
   ```

2. **Avoid Vec/String**: Use fixed-size arrays when possible

3. **Inline small functions**: Use `#[inline]` on hot path functions

4. **Minimize dependencies**: Each crate adds overhead

5. **Use cargo-bloat** to analyze what's taking up space:
   ```bash
   cargo install cargo-bloat
   cargo bloat --release --target wasm32-unknown-unknown
   ```

6. **Consider wasm-snip** to remove unnecessary functions:
   ```bash
   cargo install wasm-snip
   wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code cart.wasm -o cart.wasm
   ```
