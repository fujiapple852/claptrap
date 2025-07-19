---
title: CLI Reference
description: A reference for the Claptrap command line interface.
sidebar:
  order: 1
---

### Claptrap CLI

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

### Shell completion generation

```text
Generate shell completion

Usage: claptrap completion [OPTIONS] --spec <FILE> <SHELL>

Arguments:
  <SHELL>
          The shell to generate completions for

          [possible values: bash, elvish, fish, powershell, zsh]

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

  -o, --output <FILE>
          The output file for the completions

  -h, --help
          Print help (see a summary with '-h')
```

### Manpage generation

```text
Generate ROFF man page

Usage: claptrap man [OPTIONS] --spec <FILE>

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

  -o, --output <FILE>
          The output file for the ROFF man page

  -h, --help
          Print help (see a summary with '-h')
```

### Script generation

```text
Generate a template script

Usage: claptrap script [OPTIONS] --spec <FILE> <SHELL>

Arguments:
  <SHELL>
          The shell to generate template script for

          Possible values:
          - bash:        Bourne Again `SHell` (bash)
          - fish:        Friendly Interactive `SHell` (fish)
          - power-shell: `PowerShell`
          - zsh:         Z `SHell` (zsh)

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

  -o, --output <FILE>
          The output file for the template script

  -h, --help
          Print help (see a summary with '-h')
```

### Documentation generation

```text
Generate documentation

Usage: claptrap doc [OPTIONS] --spec <FILE>

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

  -f, --format <FORMAT>
          The format of the documentation

          [default: markdown]

          Possible values:
          - markdown: Markdown formatted output

  -o, --output <FILE>
          The output file for the documentation

  -h, --help
          Print help (see a summary with '-h')
```

### Schema generation

```text
Generate a JSON schema for the spec

Usage: claptrap schema [OPTIONS]

Options:
  -o, --output <FILE>  The output file for the schema
  -h, --help           Print help
```
