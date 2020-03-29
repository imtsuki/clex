# clex

```bash
A C99-compatible lexer written in Rust

USAGE:
    clex <source>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    The source code file
```

## How to run

First, install the Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then just run the following command:

```bash
cd clex && cargo run -- test/test.c
```
