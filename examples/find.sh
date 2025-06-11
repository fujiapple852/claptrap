#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec find.toml -- "$@")"

for i in "${!claptrap_empty[@]:-}"; do
  echo "--empty at index ${i}"
done
for val in "${claptrap_name[@]:-}"; do
  echo "--name $val"
done
for i in "${!claptrap_or[@]:-}"; do
  echo "-o at index ${i}"
done
for i in "${!claptrap_and[@]:-}"; do
  echo "-a at index ${i}"
done
