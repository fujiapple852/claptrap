#!/usr/bin/env fish

set -l spec '
  name = "myapp"
  version = "0.1.0"
  [args]
  mode = { short = "m", long = "mode" }
  protocol = { short = "p", long = "protocol" }
'

eval (printf "%s" "$spec" | $CLAPTRAP_BIN --spec - --output-format fish -- $argv | string collect)

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
