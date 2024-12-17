# Limitations

**NOTE! This is a non-exhaustive list** \
This list contains silly errors i have to fix

### You can **not** use if statements inside functions, example:
```rust
fn a(b, c) {
    if b == c {
        print("b == c");
    }
}
```
This will cause an error: ⚠️  Expected a function before '}' \
This error is pending a fix