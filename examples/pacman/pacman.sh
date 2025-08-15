#!/usr/bin/env bash

set -euo pipefail

CLAPTRAP_BIN="${CLAPTRAP_BIN:-claptrap}"
SPEC_FILE="${CLAPTRAP_SPEC:-$(dirname "$0")/spec.yaml}"

eval "$($CLAPTRAP_BIN --spec "$SPEC_FILE" -- "$@")"

# shellcheck disable=SC2154
echo "claptrap__subcommand: ${claptrap__subcommand}"
case "${claptrap__subcommand}" in
  "query")
    if [ -n "${claptrap_query_search+x}" ]; then
      for i in "${!claptrap_query_search[@]}"; do
        echo "claptrap_query_search[$i]: ${claptrap_query_search[$i]}"
      done
    fi
    if [ -n "${claptrap_query_info+x}" ]; then
      for i in "${!claptrap_query_info[@]}"; do
        echo "claptrap_query_info[$i]: ${claptrap_query_info[$i]}"
      done
    fi
    ;;
  "sync")
    if [ -n "${claptrap_sync_search+x}" ]; then
      for i in "${!claptrap_sync_search[@]}"; do
        echo "claptrap_sync_search[$i]: ${claptrap_sync_search[$i]}"
      done
    fi
    if [ -n "${claptrap_sync_info+x}" ]; then
      echo "claptrap_sync_info: ${claptrap_sync_info}"
    fi
    if [ -n "${claptrap_sync_package+x}" ]; then
      for i in "${!claptrap_sync_package[@]}"; do
        echo "claptrap_sync_package[$i]: ${claptrap_sync_package[$i]}"
      done
    fi
    ;;
esac