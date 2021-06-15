# Clover

[![crates.io](https://img.shields.io/crates/v/clover.svg)](https://crates.io/crates/clover)
[![CI](https://github.com/ippan/clover/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/ippan/clover/actions/workflows/build_and_test.yml)
![Crates.io](https://img.shields.io/crates/l/clover)

a scripting language created in Rust

still in development~

## Features
* bytecode
* first class function
* error handling

## Example

You can go to [examples](https://github.com/ippan/clover/tree/master/examples) directory for more examples

### Hello World

```ruby
function main()
    print("hello world!")
end
```

### Include other file

rectangle.luck
```ruby
public model Rectangle
    width
    height
end

implement Rectangle
    function area(this)
        this.width * this.height
    end
end
```

main.luck
```ruby
include Rectangle from "./rectangle.luck"

function main()
    local rect = Rectangle(20, 30)
    print(rect.area())
end
```

## Editor support

### Visual Studio Code

Use [Clover VSCode Support](https://github.com/ippan/vscode-clover) for code highlighting in [Visual Studio Code](https://code.visualstudio.com/)

## Integrate to your project

### Example

```rust
let result = create_state_by_filename("example/main.luck");

match result {
  Ok(mut state) => {
    state.execute();
  }
}
```

### Export native function/struct to Clover

see [clover-std](https://github.com/ippan/clover/tree/master/crates/clover-std)

## CLI

### Install

use [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install the clover-cli

```shell
cargo install clover-cli
```

### Usage

```shell
clover examples/main.luck
```