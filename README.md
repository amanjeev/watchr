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
watchr 0.1.0
Amanjeev Sethi
Execute a command periodically. Like watch(1) command.

USAGE:
    wr [OPTIONS] <command>...

ARGS:
    <command>...    Command to watch over

OPTIONS:
    -c                   Clear the terminal on each iteration
    -d <duration>        Duration in seconds to repeat the command execution [default: 1]
    -h, --help           Print help information
    -V, --version        Print version information
```

## Caveats

- Does not support error propagation if exit code is not 0 of your command
- Linux only
