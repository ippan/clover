# Clover

[![crates.io](https://img.shields.io/crates/v/clover-std.svg)](https://crates.io/crates/clover-std)
[![CI](https://github.com/ippan/clover/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/ippan/clover/actions/workflows/build_and_test.yml)
![Crates.io](https://img.shields.io/crates/l/clover)

Clover Scripting Language Standard Library

this is the standard library for [Clover](https://github.com/ippan/clover) scripting language.

still in development~

## Implemented Models
* IO
  * print
  * readline
* Random
* Math
  * pow

## Usage

```rust
clover_std_inject_to(&mut state);
```