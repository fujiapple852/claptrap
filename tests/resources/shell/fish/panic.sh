#!/usr/bin/env fish

set -l spec '
  name = "myapp"
  [args]
  # this wil cause clap to panic
  mode = { index = 2 }
'
eval (printf "%s" "$spec" | $CLAPTRAP_BIN --spec - --output-format fish -- $argv | string collect)
