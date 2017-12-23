# Grid

[![Build Status](https://travis-ci.org/lonski/grid-rs.svg?branch=master)](https://travis-ci.org/lonski/grid-rs)
[![Documentation](https://docs.rs/grid/badge.svg)](https://docs.rs/grid)

Simple utility for 2d grid of characters.

## Installation

```toml
[dependencies]
grid = "0.1.1"
```

## Examples

```rust
let mut grid = Grid::new(10, 10);

grid.set(0, 0, '.');

assert_eq!(grid.get(0, 0), Some('.'));
assert_eq!(grid.get(200, 0), None);
assert_eq!(grid.count('.'), 1);
assert_eq!(grid.count('#'), 99);

grid.fill(5, 5, '+');

assert_eq!(grid.count('+'), 100);
```

