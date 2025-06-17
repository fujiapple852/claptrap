# A toy `pacman` command

The example `bash` script `pacman.sh` reads the app spec from `spec.toml` or `spec.yaml`.

## Usage examples

Show help:

```shell
./pacman.sh --help
package manager utility

Usage: pacman <COMMAND>

Commands:
  query, -Q, --query  Query the package database.
  sync, -S, --sync    Synchronize packages.
  help                Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Show help for the `query` subcommand:

```shell
./pacman.sh query --help
Query the package database.

Usage: {query|--query|-Q} [OPTIONS]

Options:
  -s, --search <search>...  search locally installed packages for matching strings
  -i, --info <info>...      view package information
  -h, --help                Print help
```

Query a package:

```shell
./pacman.sh query -s claptrap
claptrap_query_search[0]: claptrap
```

Show help for the `sync` subcommand:

```shell
./pacman.sh sync --help
Synchronize packages.

Usage: {sync|--sync|-S} [OPTIONS] [package]...

Arguments:
  [package]...  packages

Options:
  -s, --search <search>...  search remote repositories for matching strings
  -i, --info                view package information
  -h, --help                Print help
```

Sync a package:

```shell
/pacman.sh -S claptrap
claptrap_sync_info: false
claptrap_sync_package[0]: claptrap
```

Show sync info for some packages:

```shell
./pacman.sh --sync -i claptrap trippy
claptrap_sync_info: true
claptrap_sync_package[0]: claptrap
claptrap_sync_package[1]: trippy
```
