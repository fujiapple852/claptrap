![ci](https://github.com/fujiapple852/claptrap/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/claptrap/badge.svg)](https://docs.rs/claptrap/0.1.1)
[![Crate](https://img.shields.io/crates/v/claptrap.svg)](https://crates.io/crates/claptrap/0.1.0)

# Claptrap 👏🪤

Bring the power of [`clap`](https://crates.io/crates/clap) command line parsing to shell scripts.

## Features

- **Spec driven**: define your CLI once in `toml`, `yaml` or `json` and reuse it across shells.
- **Safe evaluation**: outputs `cat` commands for usage and errors so scripts can `eval` them safely.
- **Subcommand aware**: nested subcommands produce scoped environment variables.
- **Shell quoting**: values are quoted and arrays are emitted for multi value options.
- **Generate extras**: build completions, man pages, documentation and template scripts.
- **Flexible sources**: load specs from files or `stdin`; path and format can come from environment variables.

## Clap features

- **Subcommands**: nested subcommands, optional subcommand requirements and precedence rules.
- **Argument groups**: express mutual exclusion or dependencies between related options.
- **Aliases**: hidden and visible aliases for commands and arguments.
- **Short and long flags**: standard single-character or word-style options.
- **Environment variables**: define arguments that read from the environment when not provided.
- **Default values**: set static or conditional defaults for arguments.
- **Custom help**: add before/after text and tailor help templates.
- **Conflicts and requirements**: ensure options are used in compatible combinations.
- **Typed parsing**: parse values as numbers, booleans or other typed data.
- **Color and styles**: use clap's color choices and style sheets for help output.
- **Infer subcommands**: allow abbreviated subcommand names.
- **Infer long flags**: accept partially typed long option names.
- **External subcommands**: forward unknown commands to other programs.
- **Optional positionals**: let required positionals be omitted when desired.
- **Version propagation**: share version info across subcommands or disable it entirely.
- **Display order**: control the ordering of arguments and headings in help.
- **Argument actions**: automatically set, append, count or toggle values.
- **Value hints**: suggest file paths, commands or URLs for completions.
- **Conditional defaults**: compute defaults from the presence or value of other args.
- **Multi-value parsing**: accept repeated values via delimiters or terminators.

## Examples

`myapp.sh`:

```bash
#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec myapp.toml -- "$@")"

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
```

`myapp.toml`:

```toml
name = "myapp"
version = "0.1.0"

[args]
mode = { short = "m", long = "mode" }
protocol = { short = "p", long = "protocol" }
```

Show usage (also `-h` or `--help`):

```shell
$ ./myapp.sh
Usage: myapp [OPTIONS]

Options:
  -m, --mode <mode>
  -p, --protocol <protocol>
  -h, --help                 Print help
  -V, --version              Print version
```

Show version:

```shell
$ ./myapp.sh -V
myapp 0.1.0
```

Parse arguments:

```shell
$ ./myapp.sh -m normal --protocol http
mode: normal
protocol: http
```

Error handling:

```shell
$ ./myapp.sh -m normal --protocl http
error: unexpected argument '--protocl' found

  tip: a similar argument exists: '--protocol'

Usage: myapp --mode <mode> --protocol <protocol>

For more information, try '--help'.
```

### Docker

To run Claptrap from a Docker container, you can use the following command. Make sure to mount the directory where the
application specification file is located in the container.

```shell
docker run -it -v ${PWD}:/spec fujiapple/claptrap --spec /spec/myapp.toml
```

## Installation

### Cargo

[![Crates.io](https://img.shields.io/crates/v/claptrap)](https://crates.io/crates/claptrap/0.1.0)

```shell
cargo install claptrap --locked
```

## Status

Incomplete WIP, unpublished, experimental.

## Alternatives

- [Argc](https://crates.io/crates/argc)

## License

Claptrap is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025
