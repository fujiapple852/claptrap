#!/usr/bin/env fish

set -e

$CLAPTRAP_BIN --spec - -- $argv <<'SPEC' | source
  name = "myapp"
  [args]
  # this wil cause clap to panic
  mode = { index = 2 }
SPEC
