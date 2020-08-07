# bitdecode-rs

A small library for checking bit code matches

## Example

### Decoding bit code

```rust
use bitdecode::bitdecode::*;

fn main() {
    let bitcode: u8 = 0b00001111;
    match parse_bit_u(&bitcode, "aa bb aa  bb") {
        Ok(capture) => {
            println!("{:?}", capture);
            assert_eq!(capture["a"], 0b0011);
            assert_eq!(capture["b"], 0b0011);
        }
        Err(e) => {
            assert_eq!(e, true);
        }
    }
}
```

The first argument is the bit code and the second argument is the format string.
`parse_bit_u()` returns a hash map with a collection of bits at character positions 'a' and 'b':

```rust
capture["a"] = 0b0011
capture["b"] = 0b0011
```

#### Format string

* Each hash key must be a single character
* Space `' '` is treated as separator

The following will be processed as the same format string:

```
"aabbbbaa"
"aabb bbaa"
"a a bb    bb  a a"
```

## License

3-Clause BSD License
