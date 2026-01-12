# Tauri Plugin Mongoose

This is a Tauri plugin library for Mongoose.

## Rust Setup

Add the plugin to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-mongoose = { path = "../path/to/tauri-plugin-mongoose" }
```

Initialize the plugin in your `main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mongoose::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## JavaScript/TypeScript Setup

Install the package (once published or linked):

```bash
npm install tauri-plugin-mongoose
```

Usage:

```typescript
import { ping } from 'tauri-plugin-mongoose';

ping("Hello").then(response => {
  console.log(response);
});
```
