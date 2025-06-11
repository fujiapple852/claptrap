#!/usr/bin/env bash

set -euo pipefail

eval "$($CLAPTRAP_BIN --spec - < tests/resources/myapp.toml -- "$@")"

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"

