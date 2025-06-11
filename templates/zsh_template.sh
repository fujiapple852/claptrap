#!/usr/bin/env zsh

set -euo pipefail

if ! command -v claptrap &> /dev/null; then
    echo "claptrap command not found. Please install it first, see https://claptrap.cli.rs for instructions."
    exit 1
fi

eval "$(claptrap --spec spec.toml -- "$@")"
