# `str_windows-rs`

Provides an iterator over windows of chars (as `&str`s) of a `&str`.

Does not allocate on the heap.

## Examples

```rust
use str_windows::str_windows;

let input = "s 😀😁";
let mut iter = str_windows(input, 3);
assert_eq!(iter.next(), Some("s 😀"));
assert_eq!(iter.next(), Some(" 😀😁"));
assert!(iter.next().is_none());
```
