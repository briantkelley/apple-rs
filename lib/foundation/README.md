# objc4_foundation

Idiomatic Rust bindings for Apple's Foundation framework.

## Classes

### `NSString`

The crate includes basic support for creating `NSString` instances and constants:

```rust
// compile-time constant string
string_literal!(static greeting: NSString = "Hello");

// heap allocated string
let location = NSString::from_str("Bellevue");
```
