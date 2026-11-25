# rs-format

Shared diagnostics and parser limits for Rust format libraries. `rs-format` is a dependency-free `no_std` library that also works without an allocator. It requires Rust 1.100 or later.

Parser authors implement `FormatError` on their own error types. Applications can use the same categories and locations across parsers. The implementation controls what appears in `Display`, `Debug`, and the entire `Error::source` chain. Keep user-supplied values, keys, paths, and raw backend messages out of all three.

```rust
use core::fmt;
use rs_format::{Category, FormatError, Position};

#[derive(Debug)]
struct DepthExceeded { offset: u64 }

impl fmt::Display for DepthExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nesting limit exceeded at byte {}", self.offset)
    }
}

impl core::error::Error for DepthExceeded {}

impl FormatError for DepthExceeded {
    fn category(&self) -> Category { Category::Limit }
    fn position(&self) -> Option<Position> {
        Some(Position { offset: Some(self.offset), line: None, column: None })
    }
}

let error = DepthExceeded { offset: 0 };
let diagnostic: &dyn FormatError = &error;
assert_eq!(diagnostic.category(), Category::Limit);
assert_eq!(diagnostic.position().unwrap().offset, Some(0));
```

## Limits

| Field | Default | Counts |
| --- | ---: | --- |
| `max_depth` | 128 | Open containers. A scalar has depth zero and a root container has depth one. |
| `max_document_bytes` | 16 MiB | Input bytes in one document. |
| `max_token_bytes` | 1 MiB, capped at `usize::MAX` | Encoded bytes in one token body, before decoding or expansion. |
| `max_expansion` | 32 Mi units | Cumulative format-specific expansion work in one document. |

Each bound is inclusive. Set `max_expansion` to zero to disable constructs that incur expansion cost. A parser defines its expansion units, applies the limits, and reports `Category::Limit` when a bound is exceeded. Using the shared types alone does not change a parser's behavior.

```rust
use rs_format::Limits;

let config = Limits { max_document_bytes: 8 * 1024 * 1024, ..Limits::default() };
assert_eq!(config.max_depth, 128);
```
