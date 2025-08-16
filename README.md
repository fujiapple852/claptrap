![ci](https://github.com/fujiapple852/claptrap/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/claptrap/badge.svg)](https://docs.rs/claptrap/0.3.0)
[![Crate](https://img.shields.io/crates/v/claptrap.svg)](https://crates.io/crates/claptrap/0.3.0)

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
  [args]
  name = { short = 'n', long = "name", default-value = "world" }
SPEC
)"

echo "hello, ${claptrap_name}!"
```

Show usage:

```shell
$ ./hello.sh -h
Usage: hello [OPTIONS]

Options:
  -n, --name <name>  [default: world]
  -h, --help         Print help
```

Parse arguments:

```shell
$ ./hello.sh -n fuji
hello, fuji!
```

Error handling:

```shell
$ ./hello.sh --nam fuji
./hello.sh --nam fuji
error: unexpected argument '--nam' found

  tip: a similar argument exists: '--name'

Usage: hello --name <name>

For more information, try '--help'.
```

## Features

Claptrap brings the full power of Clap command line parsing to shell scripts. Command line interface specifications can
be declared in `toml`, `yaml` or `json` and used as standalone files or embedded directly in scripts.

Claptrap Supports `bash`, `zsh`, `fish` and `PowerShell` scripts and can run on Linux, BSD, macOS, and Windows. Claptrap
can also automatically generate shell completions, man pages, markdown documentation and template scripts.

See the [documentation](https://claptrap.sh) for more details.

## Install

Claptrap can be installed from many package managers, precompiled binaries, or source.

For example, to install Claptrap from `cargo`:

```shell
cargo install claptrap --locked
```

<details>

<summary>All package managers</summary>

### Cargo

[![Crates.io](https://img.shields.io/crates/v/claptrap)](https://crates.io/crates/claptrap/0.3.0)

```shell
cargo install claptrap --locked
```

### PPA (Ubuntu)

[![Ubuntu PPA](https://img.shields.io/badge/Ubuntu%20PPA-0.3.0-brightgreen)](https://launchpad.net/~fujiapple/+archive/ubuntu/claptrap/+packages)

```shell
add-apt-repository ppa:fujiapple/claptrap
apt update && apt install claptrap
```

> ⓘ Note:
>
> Only available for Ubuntu 24.04 (`Noble`) and 22.04 (`Jammy`).

### Snap (Linux)

[![claptrap](https://snapcraft.io/claptrap/badge.svg)](https://snapcraft.io/claptrap)

```shell
snap install claptrap
```

### Homebrew (macOS)

[![homebrew version](https://img.shields.io/badge/homebrew-0.3.0-orange)](https://github.com/fujiapple852/homebrew-claptrap)

```shell
brew tap fujiapple852/claptrap && brew install claptrap
```

### Docker

[![Docker Image Version (latest by date)](https://img.shields.io/docker/v/fujiapple/claptrap)](https://hub.docker.com/r/fujiapple/claptrap/)

```shell
docker run -it fujiapple/claptrap
```

### All Repositories

[![Packaging status](https://repology.org/badge/vertical-allrepos/claptrap.svg)](https://repology.org/project/claptrap/versions)

</details>

See the [installation](https://claptrap.sh/start/installation) guide for details of how to install Claptrap on your
system.

## Goals and non-goals

Claptrap is a command line tool that wraps the `clap` library and provides integration with shell scripts. The goal is
to expose the full power of Clap to shell scripts, allowing you to define your command line interface in a declarative
way.

If you prefer to parse command line arguments using native shell scripts then you should consider using
an [alternative](#alternatives) tools. Adding support for generating shell scripts that parse command line arguments is
a non-goal for Claptrap.

## Alternatives

- [Argc](https://crates.io/crates/argc)

## License

Claptrap is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025
