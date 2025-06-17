#!/usr/bin/env bash

set -euo pipefail

CLAPTRAP_BIN="${CLAPTRAP_BIN:-claptrap}"
SPEC_FILE="${CLAPTRAP_SPEC:-$(dirname "$0")/spec.yaml}"

# shellcheck disable=SC1090
eval "$($CLAPTRAP_BIN --spec "$SPEC_FILE" -- "$@")"

for var in $(compgen -A variable | grep '^claptrap' | sort); do
    if declare -p "$var" 2>/dev/null | grep -q 'declare -a'; then
        eval "vals=(\"\${$var[@]}\")"
        for i in "${!vals[@]}"; do
            echo "$var[$i]: ${vals[$i]}"
        done
    else
        eval "val=\${$var}"
        echo "$var: $val"
    fi
done