---
title: CLI Reference
description: A reference for the Claptrap command line interface.
sidebar:
  order: 1
---

```text
Bring the power of Clap to shell scripts.

Usage: claptrap [OPTIONS] --spec <FILE> [-- <ARGS>...]
       claptrap <COMMAND>

Commands:
  completion  Generate shell completion
  man         Generate ROFF man page
  script      Generate a template script
  doc         Generate documentation
  schema      Generate a JSON schema for the spec
  help        Print this message or the help of the given subcommand(s)

Arguments:
  [ARGS]...
          Arguments to pass to the command

Options:
  -s, --spec <FILE>
          Sets a custom config file

          [env: CLAPTRAP_SPEC=]

      --spec-format <FORMAT>
          The format of the spec file

          [env: CLAPTRAP_SPEC_FORMAT=]
          [default: auto]

          Possible values:
          - auto: Automatically detect the spec format
          - json: JSON spec format
          - yaml: YAML spec format
          - toml: TOML spec format

      --show-panic
          Do not suppress panic messages

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
