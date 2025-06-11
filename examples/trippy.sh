#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec trip.toml -- "$@")"

echo "protocol: $claptrap_protocol"
echo "multi[0]: ${claptrap_multi[0]}"
echo "multi[1]: ${claptrap_multi[1]}"
