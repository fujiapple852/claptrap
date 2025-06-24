![ci](https://github.com/fujiapple852/claptrap/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/claptrap/badge.svg)](https://docs.rs/claptrap/0.1.1)
[![Crate](https://img.shields.io/crates/v/claptrap.svg)](https://crates.io/crates/claptrap/0.1.0)

# Claptrap 👏🪤

Bring the power of [Clap](https://crates.io/crates/clap) to shell scripts.

Claptrap is a tool that allows you to parse complex command line arguments in shell scripts using a declarative
specification.

## Example

`hello.sh`:

```bash
#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec - -- "$@" <<'SPEC'
  name = "hello"
  version = "0.1.0"
  arg-required-else-help = true
  [args]
  format = { short = "f", long = "format", required = true, value-parser = ["toml", "yaml", "json"] }
  exts = { long = "extensions", num-args = "1..", default-values = ["txt", "sh", "rs"] }
SPEC
)"

echo "format: $claptrap_format"
for i in "${!claptrap_exts[@]}"; do
  echo "extensions[$i]: ${claptrap_exts[$i]}"
done
```

Show usage:

```shell
$ ./hello.sh
Usage: hello [OPTIONS] --format <format>

Options:
  -f, --format <format>       [possible values: toml, yaml, json]
      --extensions <exts>...  [default: txt sh rs]
  -h, --help                  Print help
  -V, --version               Print version
```

Parse arguments:

```shell
$ ./hello.sh -f json
protocol: json
extensions[0]: txt
extensions[1]: sh
extensions[2]: rs
```

Error handling:

```shell
$ ./hello.sh -f yml --extensions md ts js
error: invalid value 'yml' for '--format <format>'
  [possible values: toml, yaml, json]

  tip: a similar value exists: 'yaml'

For more information, try '--help'.
```

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

## Installation

### Cargo

[![Crates.io](https://img.shields.io/crates/v/claptrap)](https://crates.io/crates/claptrap/0.1.0)

```shell
cargo install claptrap --locked
```

## Goals and non-goals

Claptrap is a command line tool that wraps the `clap` library and provides integration with shell scripts. The goal is
to expose the full power of Clap to shell scripts, allowing you to define your command line interface in a declarative
way.

If you prefer to parse command line arguments using native shell scripts then you should consider using
an [alternative](#alternatives) tools. Adding support for generating shell scripts that parse command line arguments is
a non-goal for Claptrap.

## Alternatives

- [Argc](https://crates.io/crates/argc)

## Status

Incomplete WIP, unpublished, experimental.

## License

Claptrap is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025
