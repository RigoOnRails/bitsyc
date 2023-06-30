# bitsyc
[![crates.io](https://img.shields.io/crates/v/bitsyc.svg)](https://crates.io/crates/bitsyc)
[!["Lint & run tests" workflow](https://github.com/RigoOnRails/bitsyc/actions/workflows/development.yml/badge.svg)](https://github.com/RigoOnRails/bitsyc/actions/workflows/development.yml)

`bitsyc` is a compiler for the Bitsy programming language. Bitsy is a small, not-very-useful language. Its primary purpose is to be the best language to implement for programmers wanting to build a compiler or interpreter for the first time.

You can find the language spec document here: [@apbendi/bitsyspec](https://github.com/apbendi/bitsyspec/blob/master/BITSY.md).

## Installation
```console
$ cargo install bitsyc
```

## Compiling a Bitsy program
This will compile down to machine code for your platform. The code generation is done using LLVM.
```console
$ bitsyc example.bitsy
```

To execute your code, just run:
```console
$ ./example
```
