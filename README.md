# Tauri Plugin Mongoose

A Tauri v2 plugin for MongoDB/Mongoose-like database operations.

## Installation

### Rust (src-tauri/Cargo.toml)

```toml
[dependencies]
tauri-plugin-mongoose = "0.2.1"
```

### JavaScript/TypeScript

```bash
npm install tauri-plugin-mongoose
```

## Setup

### 1. Register the plugin in your Tauri app

In your `src-tauri/src/lib.rs`:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mongoose::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Add permissions in tauri.conf.json

In your `src-tauri/tauri.conf.json`, add the plugin permissions:

```json
{
  "plugins": {
    "mongoose": {
      "permissions": ["allow-connect", "allow-create", "allow-get-by-id"]
    }
  }
}
```

Or use the default permissions:

```json
{
  "plugins": {
    "mongoose": "default"
  }
}
```

## Usage

```typescript
import { connect, Model } from 'tauri-plugin-mongoose';

// Connect to MongoDB
await connect('mongodb://localhost:27017', 'myDatabase');
// Or with options object
await connect({ url: 'mongodb://localhost:27017', dbName: 'myDatabase' });

// Define a schema
const userSchema = {
  name: { type: 'string', required: true },
  email: { type: 'string', required: true, unique: true },
  age: { type: 'number' }
};

// Create a model
const User = new Model('users', userSchema);

// Create a document
const newUser = await User.create({
  name: 'John Doe',
  email: 'john@example.com',
  age: 30
});

// Get document by ID
const user = await User.getById('60f1b5d6e4b0e...');
```

## API

### `connect(url: string, dbName?: string): Promise<void>`
### `connect(options: ConnectOptions): Promise<void>`

Connect to a MongoDB database.

### `Model`

A class for defining and interacting with MongoDB collections.

- `new Model(name: string, schema: Schema)` - Create a new model
- `model.create(doc)` - Create a new document
- `model.getById(id)` - Get a document by its ObjectId

## License

MIT
