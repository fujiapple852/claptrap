#!/usr/bin/env fish

eval ( $CLAPTRAP_BIN --shell fish --spec $CLAPTRAP_SPEC -- $argv | string collect )

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
