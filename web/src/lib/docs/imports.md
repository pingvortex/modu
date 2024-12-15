# Imports

We currently only support importing other files, no packages yet.

```rust
// yapper.modu

fn yap(msg) {
    print(msg);
}

// main.modu

import "yapper.modu" as yapper

yapper.yap("Hello, World!");

// Outputs
//
// Hello, World!
```