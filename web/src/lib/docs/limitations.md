# Limitations

**NOTE! This is a non-exhaustive list** \
This list contains silly errors i have to fix.

### Nesting functions is a little goofy :(
> And unpredictable

Stuff like:

```rust
import "math" as m

m.div(m.div(1,2), m.div(1,2))
```
Will most likely break. \
You can get around this by using variables:
```rust
import "math" as m

let a = m.div(5,2)
let b = m.div(1,2)

m.div(a, b)
```

Status: **To be Fixed**