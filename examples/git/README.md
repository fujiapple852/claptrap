# A toy `git` command

The example `bash` script `git.sh` reads the app spec from `spec.toml`.

## Usage examples

Show help:

```shell
./git.sh --help
A toy versioning of git

Usage: git <COMMAND>

Commands:
  clone  Clones repos
  diff   Compare two commits
  push   pushes things
  add    adds things
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Clone a repository:

```shell
./git.sh clone https://github.com/fujiapple852/claptrap.git
claptrap__subcommand: clone
claptrap_clone_REMOTE: https://github.com/fujiapple852/claptrap.git
```

Diff between two commits for a specific path:

```shell
./git.sh diff 9561142 1dfd1b5 --color=never -- some/path/
claptrap__subcommand: diff
claptrap_diff_base: 9561142
claptrap_diff_color: never
claptrap_diff_head: 1dfd1b5
claptrap_diff_path: some/path/
```

Push:

```shell
./git.sh push origin
claptrap__subcommand: push
claptrap_push_REMOTE: origin
```

Add:

```shell
./git.sh add main.rs lib.rs error.rs Cargo.toml
claptrap__subcommand: add
claptrap_add_PATH[0]: main.rs
claptrap_add_PATH[1]: lib.rs
claptrap_add_PATH[2]: error.rs
claptrap_add_PATH[3]: Cargo.toml
```
