# Clover

| clover | clover-std | clover-cli |
| ----------- | ------- | ----------- |
| [![crates.io](https://img.shields.io/crates/v/clover.svg)](https://crates.io/crates/clover) | [![crates.io](https://img.shields.io/crates/v/clover-std.svg)](https://crates.io/crates/clover-std)  | [![crates.io](https://img.shields.io/crates/v/clover-cli.svg)](https://crates.io/crates/clover-cli)  |

[![CI](https://github.com/ippan/clover/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/ippan/clover/actions/workflows/build_and_test.yml)
![Crates.io](https://img.shields.io/crates/l/clover)

a scripting language created in Rust

still in development~

## Features
* bytecode
* first class function

## Example

```ruby
include Vector2D as Vector from "./vector"

public model Rect
  start
  size
end

implement Rect
  function new()
    local rect = Rect()
    rect.start = Vector.new()
    rect.size = Vector.new()
    rect
  end
end

model MyRect
end

# copy all function in Rect to MyRect
apply Rect to MyRect

function main()
  local rect = MyRect.new()
  rect
end
```

## Integrate to your project

```rust
let result = create_state_by_filename("example/main.luck");

match result {
  Ok(mut state) => {
    state.execute();
  }
}
```

## CLI

```sh
clover examples/main.luck
```