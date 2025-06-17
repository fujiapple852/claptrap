#!/usr/bin/env bash

set -euo pipefail

eval "$(docker run -it -v ${PWD}:/data fujiapple/claptrap:latest --spec /data/spec.toml -- "$@")"

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