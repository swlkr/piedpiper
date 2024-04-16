# Piedpiper

## Gleam style pipes in rust!

```rust
use piedpiper::pp;

fn add(x: u64, y: u64) -> u64 {
  x + y
}

let result = pp! {
  1
  |> add(_, 2)
  |> add(_, 3)
};

assert_eq!(result, 6)
```
