#!/usr/bin/env bash

set -euo pipefail

eval "$($CLAPTRAP_BIN --spec - -- "$@" <<'SPEC'
  name = "myapp"
  version = "0.1.0"
  [args]
  mode = { short = "m", long = "mode" }
  protocol = { short = "p", long = "protocol" }
SPEC
)"

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"

