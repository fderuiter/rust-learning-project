# WebAssembly Integration

This document describes how to create and use WebAssembly (Wasm) functions in this project.

## Creating a Wasm Function

To create a function that can be called from JavaScript, you need to:

1.  Define the function in Rust.
2.  Annotate it with `#[wasm_bindgen]`.

Here is an example of a simple `add` function in `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Calling a Wasm Function from JavaScript

To call a Wasm function from JavaScript, you need to:

1.  Import the Wasm module.
2.  Initialize the module.
3.  Call the exported function.

Here is an example of how to call the `add` function in `main.js`:

```javascript
import init, { add } from './rust_learning_project.js';

async function run() {
  // Initialize the Wasm module.
  await init();

  // Call the Wasm function and log the result.
  const result = add(2, 3);
  console.log(`Hello from Wasm! 2 + 3 = ${result}`);
}

run();
```

The `trunk` build tool will automatically handle the compilation and bundling of the Wasm module, making it available for import in your JavaScript code.
