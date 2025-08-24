#!/usr/bin/env fish

eval ($CLAPTRAP_BIN --spec - --spec-format $CLAPTRAP_SPEC_FORMAT --output-format fish -- $argv < $CLAPTRAP_SPEC | string collect)

echo "mode: $claptrap_mode"
echo "protocol: $claptrap_protocol"
