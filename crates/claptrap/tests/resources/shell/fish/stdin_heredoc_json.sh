#!/usr/bin/env fish

set -l spec '
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
'
eval (printf "%s" "$spec" | $CLAPTRAP_BIN --spec - --output-format fish -- $argv | string collect)

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
