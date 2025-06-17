# Examples

Examples of using `claptrap` to create toy command line applications.

The examples use claptrap from either `bash` or `zsh` and the application specifications are defined in `toml`, `yaml`
or `json`.

### Example 1: `git`

A toy `git` command with several subcommands like `clone`, `diff`, `push`, and `add`.

- Shell: `bash`
- App spec format: `toml`

### Example 2: `pacman`

A toy `pacman` command with subcommands like `query` and `sync`.

- Shell: `bash`
- App spec format: `toml` & `yaml`

### Example 3: `docker`

A toy `docker` command with subcommands like `run`.

- Shell: `bash`
- App spec format: `toml`

This example runs `claptrap` in a Docker container, so you need to have Docker installed and running.
