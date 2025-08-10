#!/usr/bin/env fish

eval ($CLAPTRAP_BIN --spec $CLAPTRAP_SPEC --output-format fish -- $argv | string collect)

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
