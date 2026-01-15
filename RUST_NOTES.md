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


