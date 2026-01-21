# 🚀 Leptos Quick Start Guide

## Instalasi Cepat (5 menit)

```bash
# 1. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install trunk (build tool)
cargo install trunk

# 3. Add WASM target
rustup target add wasm32-unknown-unknown

# 4. Create project
cargo new leptos-rust-ui
cd leptos-rust-ui
```

## File Structure

```
leptos-rust-ui/
├── src/
│   ├── main.rs          # App entry point
│   └── components.rs    # Reusable components
├── Cargo.toml           # Dependencies & config
├── index.html           # HTML template
└── README.md
```

## Minimal Example (30 detik)

### src/main.rs
```rust
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <h1>"Counter: " {count}</h1>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "+"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
```

### Cargo.toml
```toml
[package]
name = "leptos-rust-ui"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
```

## Run Development Server

```bash
# Start with hot reload
trunk serve

# Open browser
# http://localhost:8080
```

## Build for Production

```bash
# Optimized build
trunk build --release

# Output in dist/
# - index.html
# - *.wasm (binary)
# - *.js (loader)

# Deploy dist/ folder to any static host
```

## Common Commands

```bash
# Development
trunk serve                 # Start dev server
trunk serve --open          # Start + open browser
trunk serve --port 3000     # Custom port

# Production
trunk build --release       # Optimized build
trunk clean                 # Clean build artifacts

# Testing
cargo test                  # Run unit tests
wasm-pack test --headless   # Run browser tests
```

## Debugging Tips

```bash
# Check WASM size
ls -lh dist/*.wasm

# Analyze bundle
cargo install twiggy
twiggy top target/wasm32-unknown-unknown/release/*.wasm

# Profile build time
cargo build --release --timings
```

## Next Steps

1. ✅ Read full README.md untuk deep dive
2. ✅ Experiment dengan components.rs
3. ✅ Add Tailwind CSS untuk styling
4. ✅ Explore [Leptos Book](https://book.leptos.dev/)
5. ✅ Check [Examples](https://github.com/leptos-rs/leptos/tree/main/examples)

## Troubleshooting

**Error: `wasm32-unknown-unknown` not found**
```bash
rustup target add wasm32-unknown-unknown
```

**Error: `trunk` command not found**
```bash
cargo install trunk --locked
```

**WASM file too large (>500KB)**
```bash
# Add to Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

**Hot reload not working**
```bash
# Restart trunk
trunk clean
trunk serve
```

---

Happy coding with Rust! 🦀