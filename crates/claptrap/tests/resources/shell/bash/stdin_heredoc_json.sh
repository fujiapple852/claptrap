#!/usr/bin/env bash

set -euo pipefail

eval "$($CLAPTRAP_BIN --spec - -- "$@" <<'SPEC'
  {
    "name": "myapp",
    "version": "0.1.0",
    "args": {
      "mode": {
        "long": "mode",
        "short": "m"
      },
      "protocol": {
        "long": "protocol",
        "short": "p"
      }
    }
  }
SPEC
)"

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"

