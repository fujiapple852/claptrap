#!/usr/bin/env fish

set -e

$CLAPTRAP_BIN --spec $CLAPTRAP_SPEC -- $argv | source

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
