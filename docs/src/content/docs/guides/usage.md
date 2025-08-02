---
title: Using Claptrap
description: A guide to using Claptrap.
sidebar:
  order: 1
---

## Introduction

Claptrap takes a command line interface specification and generates environment variables which represent parsed
command line arguments.

Claptrap is designed to be run inside an `eval` command and ensures that all output to standard output is in the form of
environment variable assignments or eval-safe shell commands such as `cat`.

In the following example, the `claptrap` command reads a specification from the file `spec.toml` and the command line
arguments passed to the script in `$@`. It will parse the command line arguments according to the specification and
output environment variable assignments that can be used in the script.

```shell
eval "$(claptrap --spec spec.toml -- "$@")"
```

When run, it will output the following environment variable assignments which will be set by the `eval` command and can
be used in the script. For example, if the specification file contains an argument named `name` with a default value of
`world`, the output will be:

```shell
claptrap_name="world"
```

> **Note:** The `claptrap_` prefix is used to avoid conflicts with other environment variables in the script.

## Specification files

Claptrap specification files are used to define the command line interface of your script. The specification file
maps closely to the API provided by the [Clap](https://crates.io/crates/clap) library.

Specification files can be written in various formats and can be embedded directly in your script or stored in a
standalone file.

The following example shows a simple `toml` specification that:

- is named `simple` and has version `0.1.0`
- defines a required argument `format` which has both short and long form argument names and requires a single value
  that must be one of `toml`, `yaml`, or `json`
- defines an optional argument `exts` that has a long form argument name, accepts one or more values and has a default
  value of `["txt", "sh", "rs"]`

```toml
name = "simple"
version = "0.1.0"
[args]
format = { short = "f", long = "format", required = true, value-parser = ["toml", "yaml", "json"] }
exts = { long = "extensions", num-args = "1..", default-values = ["txt", "sh", "rs"] }
```

See the [API Reference](/reference/api) for details of the specification format and examples.

### Specification formats

Claptrap supports command line interface specifications in `toml`, `yaml`, and `json` formats.

Claptrap will attempt to automatically detect the format of the specification file you provide.

If you are using a specification file with a `.toml`, `.yaml`, or `.json` extension, Claptrap will automatically detect
the format based on the file extension.

If you pass the specification file via standard input or heredoc, Claptrap will attempt to detect the format based on
the content of the file.

If necessary you may use the `--spec-format` option to specify the format explicitly.

### Parsing command line arguments

Parsing command line arguments with Claptrap is done by running the `claptrap` command and passing the specification
file and the command line arguments to it.

Claptrap can read a specification from the path given in the `--spec` command line argument:

```shell
eval "$(claptrap --spec spec.toml -- "$@")"
```

Claptrap can also read a specification file from standard input via redirect or a pipe:

```shell
eval "$(claptrap --spec - < spec.toml -- "$@")"
eval "$(cat spec.toml | claptrap --spec - -- "$@")"
```

Finally, Claptrap can read the specification from a heredoc:

```shell
eval "$(claptrap --spec - -- "$@" <<'SPEC'
  name = "hello"
  [args]
  name = { short = 'n', long = "name", default-value = "world" }
SPEC
)"
```

The command line arguments that should be parsed by Claptrap are passed to the `claptrap` command after the `--`
argument. Typically, this is the `$@` variable which contains all the command line arguments passed to the script.

## Docker

Claptrap can be run inside a Docker container. This is useful for running Claptrap in a controlled environment or
when you want to avoid [installing](/start/installation) Claptrap on your host system.

The following example shows how to run Claptrap inside a Docker container:

```bash
#!/usr/bin/env bash

set -euo pipefail

eval "$(docker run -it -v ${PWD}:/data fujiapple/claptrap:latest --spec spec.toml -- "$@")"
```

The Claptrap [Docker image](/start/installation/#docker) expects the specification file to be mounted in the `/data/`
directory inside the container.

## Generation script

Claptrap can generate a template script based on the command line interface specification.

```shell
claptrap script --spec spec.toml bash -o hello.sh
```

The generated `hello.sh`:

```bash
#!/usr/bin/env bash

set -euo pipefail

if ! command -v claptrap &> /dev/null; then
    echo "claptrap command not found. Please install it first, see https://claptrap.sh for instructions."
    exit 1
fi

eval "$(claptrap --spec spec.toml -- "$@")"

if [ -n "${claptrap_name+x}" ]; then
  echo "claptrap_name: ${claptrap_name}"
fi
```

## Generate man pages

Claptrap can generate man pages in `roff` format from a command line interface specification file.

```shell
claptrap man --spec spec.toml -o myapp.roff
```

## Generate shell completions

Claptrap can generate shell completions for various shells from a command line interface specification file.

```shell
claptrap completion --spec spec.toml --shell bash -o myapp.bash
claptrap completion --spec spec.toml --shell zsh -o myapp.zsh
claptrap completion --spec spec.toml --shell fish -o myapp.fish
claptrap completion --spec spec.toml --shell powershell -o myapp.ps1
```

## Generate markdown documentation

Claptrap can generate markdown documentation from a command line interface specification file.

```shell
claptrap doc --spec spec.toml -o myapp.md
```

## Generate JSON schema

Claptrap can generate a JSON schema for the command line interface specification format.

```shell
claptrap schema -o schema.json
```
