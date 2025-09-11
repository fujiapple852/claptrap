#!/usr/bin/env bash

set -euo pipefail

eval "$($CLAPTRAP_BIN --spec - -- "$@" <<'SPEC'
  name = "myapp"
  [args]
  # this wil cause clap to panic
  mode = { index = 2 }
SPEC
)"
