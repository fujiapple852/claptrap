#!/usr/bin/env fish

set spec "name = \"myapp\"
[args]
# this wil cause clap to panic
mode = { index = 2 }
"
eval (printf "%s" "$spec" | $CLAPTRAP_BIN --shell fish --spec - -- $argv | string collect)
