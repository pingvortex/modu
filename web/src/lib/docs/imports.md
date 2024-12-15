# Imports

We currently only support importing other files, no packages yet. \
When you import a file, you can access it's variables and functions using pkg.*, if you imported with "... as pkg". \
'pkg' can be any non-reserved keyword.

```rust
// yapper.modu

let abc = 123;

fn yap(msg) {
    print(msg);
}

```

<span class="my-5" ></span>

```rust
// main.modu

import "yapper.modu" as yapper

yapper.yap("Hello, World!");
print(yapper.abc);
```

This should output:
```
Hello, World!
123
```