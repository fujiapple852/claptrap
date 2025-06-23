![ci](https://github.com/fujiapple852/claptrap/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/claptrap/badge.svg)](https://docs.rs/claptrap/0.1.1)
[![Crate](https://img.shields.io/crates/v/claptrap.svg)](https://crates.io/crates/claptrap/0.1.0)

# Claptrap 👏🪤

Bring the power of [Clap](https://crates.io/crates/clap) to shell scripts.

Claptrap is a tool that allows you to parse complex command line arguments in shell scripts from a specification file.

## Features

- **Spec driven**: define your CLI once in `toml`, `yaml` or `json` and reuse it across shells.
- **Shell agnostic**: works with `bash` and `zsh` with more to come (`fish`, `PowerShell` and others).
- **Platform independent**: runs on Linux, macOS, Windows and BSD.
- **Safe evaluation**: only outputs `eval`-safe commands for usage and errors.
- **Shell quoting**: values are quoted and arrays are emitted for multi value options.
- **Generate extras**: generate shell completions, man pages, documentation and template scripts.
- **Flexible sources**: load specs from files, `stdin` or from heredoc

## Parsing features

Claptrap supports the full range of Clap features, including:

- **Short and long flags**: standard single-character or word-style options.
- **Multi-value parsing**: accept repeated values via delimiters or terminators.
- **Subcommands**: nested subcommands, optional subcommand requirements and precedence rules.
- **Argument groups**: express mutual exclusion or dependencies between related options.
- **Default values**: set static or conditional defaults for arguments.
- **Typed parsing**: parse values as numbers, booleans or other typed data.
- **Custom help**: add before/after text and tailor help templates.
- **Color and styles**: use clap's color choices to style help output.
- **Value hints**: suggest file paths, commands or URLs for completions.
- **Environment variables**: define arguments that read from the environment when not provided.

See the full list of [supported](https://claptrap.cli.rs/reference/supported/) Clap features.

## Example

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

## Installation

### Cargo

[![Crates.io](https://img.shields.io/crates/v/claptrap)](https://crates.io/crates/claptrap/0.1.0)

```shell
cargo install claptrap --locked
```

## Status

Incomplete WIP, unpublished, experimental.

## Goals and non-goals

Claptrap is not an argument parser, but rather a command line tool that wraps the `clap` library and provides
integration with shell scripts. The goal is to expose the full power of Clap to shell scripts, allowing you to define
your command line interface in a declarative way.

If you are unable to install the `claptrap` binary, or you prefer to parse command line arguments using native shell
scripts, you should consider using alternative tools, see the [alternatives](#alternatives) section. Adding support for
generating shell scripts that parse command line arguments is a non-goal Claptrap.

## Alternatives

- [Argc](https://crates.io/crates/argc)

## License

Claptrap is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025
