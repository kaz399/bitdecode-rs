# bitdecode-rs

A library for decoding bit patterns

## Example

```
capture = parse_bit(0b00011011, "aabbbbaa").unwrap()
```

The first argument is the bit code and the second argument is the format string.
`parse_bit()` returns a hash map with a collection of bits at character positions 'a' and 'b':

```
capture["a"] = 0b0011
capture["b"] = 0b1110
```

### About the format string

* Each hash key must be a single character
* ' ', '_' and '|' are treated as separators. (Thiese caracters can't be used as hash keys)

The following will be processed as the same format string:

```
"aabbbbaa"
"aabb bbaa"
"aabb_bbaa"
"aabb|bbaa"
"aabb    bbaa"
```

## License

3-Clause BSD License
