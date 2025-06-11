#!/usr/bin/env bash

set -euo pipefail

eval "$($CLAPTRAP_BIN --spec-format $CLAPTRAP_SPEC_FORMAT --spec - < $CLAPTRAP_SPEC -- "$@")"

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"

