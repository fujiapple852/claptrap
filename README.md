![ci](https://github.com/fujiapple852/claptrap/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/claptrap/badge.svg)](https://docs.rs/claptrap/0.1.1)
[![Crate](https://img.shields.io/crates/v/claptrap.svg)](https://crates.io/crates/claptrap/0.1.0)

# Claptrap 👏🪤

Bring the power of [`clap`](https://crates.io/crates/clap) command line parsing to shell scripts.

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
