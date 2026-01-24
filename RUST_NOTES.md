## Rust CLI basics

### Running a Rust program

- **Using Cargo (recommended for projects)**
  - `cargo run`  
    - Builds (if needed) and runs the project in **debug** mode.  
    - Uses `target/debug/your_binary_name`.
  - `cargo run --release`  
    - Builds and runs in **release** mode with optimizations (faster runtime, slower compile).  
    - Uses `target/release/your_binary_name`.
  - `cargo build` / `cargo build --release`  
    - Only builds the binary (debug or release), does **not** run it.
    - You can then run directly:
      - Debug: `./target/debug/your_binary_name`
      - Release: `./target/release/your_binary_name`
    - Useful when you want to run the **same binary multiple times** without re-running Cargo; you just execute the already-compiled file.

- **Using `rustc` directly**
  - For simple, single-file examples (no `Cargo.toml` or dependencies):
    - `rustc main.rs`
    - `./main`
  - In real projects (with Cargo), you normally **don’t** use `rustc` yourself; Cargo calls it for you.

### The `target` directory

- Created the first time you build or run with Cargo:
  - `cargo build`, `cargo run`, `cargo test`, etc.
- Contains compiled artifacts:
  - `target/debug/` → debug builds (`cargo run`, `cargo build`)
  - `target/release/` → release builds (`cargo run --release`, `cargo build --release`)
- Safe to delete; Cargo will recreate it on the next build.

### Testing with Cargo

- `cargo test`
  - Finds and runs tests:
    - Functions annotated with `#[test]` (usually inside `#[cfg(test)] mod tests { ... }`).
    - Integration tests in `tests/` directory (if present).
  - Builds your code in **test mode** and reports which tests passed/failed.

### Rule of thumb: Cargo vs `rustc`

- **Use Cargo** (`cargo run`, `cargo build`, `cargo test`) for any normal project with `Cargo.toml`.
- **Use `rustc`** mainly for tiny, stand‑alone examples or when you specifically want to experiment with compiler flags.

## Dependencies vs Global Tools

### Adding dependencies to your project (`Cargo.toml`)

Dependencies are libraries your code uses. They go in `Cargo.toml` under `[dependencies]`.

**Method 1: `cargo add` (recommended)**
```bash
cargo add serde
cargo add tokio --features full
cargo add serde@1.0
```
- Automatically adds to `Cargo.toml`
- Updates `Cargo.lock`
- Downloads the crate

**Method 2: Manual editing**
Edit `Cargo.toml` directly:
```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```
Then run `cargo build` to update `Cargo.lock`.

### Installing global tools (`cargo install`)

Tools installed via `cargo install` are **NOT** project dependencies:
- Installed globally to `~/.cargo/bin/`
- Available system-wide (if `~/.cargo/bin` is in PATH)
- **Do NOT** appear in `Cargo.toml` or `Cargo.lock`
- Used for development tools and CLI utilities

**Examples:**
```bash
cargo install cargo-watch    # Development tool
cargo install cargo-llvm-cov # Coverage tool
cargo install ripgrep        # CLI utility
```

**Note:** Some tools like `cargo-llvm-cov` require additional Rust components:
```bash
rustup component add llvm-tools-preview  # or llvm-tools
```

### Key differences

| Command | Purpose | Goes in `Cargo.toml`? | Location |
|---------|---------|----------------------|----------|
| `cargo add serde` | Add library dependency | ✅ Yes | `[dependencies]` |
| `cargo install cargo-watch` | Install global tool | ❌ No | `~/.cargo/bin/` |

**Summary:**
- **`cargo add`** → Project dependencies (libraries your code uses)
- **`cargo install`** → Global tools (CLI utilities you run)

## Crate Features

### What are features?

Features are **optional modules** you can enable in `Cargo.toml`. They let you include only what you need, reducing compile time and binary size.

### Common Tokio features (example)

| Feature | What it enables | When you need it |
|---------|------------------|-------------------|
| `macros` | `#[tokio::main]`, `#[tokio::test]` | Using `#[tokio::main]` attribute |
| `rt` | Single-threaded runtime | Basic async runtime |
| `rt-multi-thread` | Multi-threaded runtime | Parallel execution (web servers) |
| `net` | TCP/UDP/Unix sockets | Network I/O operations |
| `fs` | Async file operations | Async file reading/writing |
| `time` | Timers, delays, intervals | Timeouts, scheduling tasks |
| `sync` | Channels, mutexes, etc. | Inter-task communication |
| `full` | All features enabled | Quick prototyping (larger compile) |

### How to know which features to add

**Method 1: Compiler errors (most reliable)**
- The compiler will tell you what's missing:
  ```
  error: The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.
  ```
- Add the feature mentioned in the error

**Method 2: Check what your code uses**
- `#[tokio::main]` → needs `macros`
- Web server (actix-web) → needs `rt-multi-thread` for parallelism
- Network operations → needs `net`
- File operations → needs `fs`
- Timers/delays → needs `time`

**Method 3: Check crate documentation**
- Visit `https://docs.rs/<crate-name>` or `https://crates.io/crates/<crate-name>`
- Look for "Feature flags" or "Features" section
- Each module's docs mention required features

**Method 4: Use `full` for prototyping**
- Start with `features = ["full"]` for quick prototyping
- Later, trim down based on what you actually use

### Example: Adding features

```bash
# Add with specific features
cargo add tokio --features macros,rt,rt-multi-thread

# Or edit Cargo.toml directly
[dependencies]
tokio = { version = "1.49.0", features = ["macros", "rt", "rt-multi-thread"] }

# For prototyping, use full
tokio = { version = "1.49.0", features = ["full"] }
```

### Quick reference

1. **Check docs**: Visit `https://docs.rs/<crate>` → "Feature flags"
2. **Check errors**: Compiler messages often indicate missing features
3. **Use `full`**: Start with `features = ["full"]`, then optimize later
4. **Check examples**: See what similar projects use

