# foundation

Idiomatic Rust bindings for Apple's Foundation framework.

## Classes

The crate includes support for creating and using instances of:

* `NSDictionary` and `NSMutableDictionary`
* `NSNumber`
* `NSString` (including compile-time constants)

```rust
string_literal!(static LOCATION: NSString = "location"); // compile-time constant
let location = NSStringClass.from_str("Bellevue");       // heap allocated

let mut dict = NSMutableDictionary::<NSString, NSString>::new();
dict.set(LOCATION, location);
assert_eq!(dict.len(), 1);
```
