# watchr

This is a tool similar to Linux [`watch(1)`](https://man7.org/linux/man-pages/man1/watch.1.html)
which runs your command periodically.  The enhancement in this tool over that one is that this
one allows you to see the history of the runs if you want to compare your command's current output
with an iteration a few seconds ago.

## Installation

`cargo install watchr`

This will deploy a binary named `wr` in your `.cargo` directory.

## Usage

```shell
Incomplete `GNU cat` in Rust for learning purposes

Usage: wr [OPTIONS] <command>

Arguments:
  <command>

Options:
  -c
  -d <duration>      [default: 1]
  -h, --help         Print help
  -V, --version      Print version
```

## Caveats

- Does not support error propagation if exit code is not 0 of your command
- Linux only
