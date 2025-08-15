#!/usr/bin/env bash

set -euo pipefail

CLAPTRAP_BIN="${CLAPTRAP_BIN:-claptrap}"
SPEC_FILE="${CLAPTRAP_SPEC:-$(dirname "$0")/spec.toml}"

eval "$($CLAPTRAP_BIN --spec "$SPEC_FILE" -- "$@")"

# shellcheck disable=SC2154
echo "claptrap__subcommand: ${claptrap__subcommand}"
case "${claptrap__subcommand}" in
  "clone")
    if [ -n "${claptrap_clone_REMOTE+x}" ]; then
      echo "claptrap_clone_REMOTE: ${claptrap_clone_REMOTE}"
    fi
    ;;
  "diff")
    if [ -n "${claptrap_diff_base+x}" ]; then
      echo "claptrap_diff_base: ${claptrap_diff_base}"
    fi
    if [ -n "${claptrap_diff_head+x}" ]; then
      echo "claptrap_diff_head: ${claptrap_diff_head}"
    fi
    if [ -n "${claptrap_diff_path+x}" ]; then
      echo "claptrap_diff_path: ${claptrap_diff_path}"
    fi
    if [ -n "${claptrap_diff_color+x}" ]; then
      echo "claptrap_diff_color: ${claptrap_diff_color}"
    fi
    ;;
  "push")
    if [ -n "${claptrap_push_REMOTE+x}" ]; then
      echo "claptrap_push_REMOTE: ${claptrap_push_REMOTE}"
    fi
    ;;
  "add")
    if [ -n "${claptrap_add_PATH+x}" ]; then
      for i in "${!claptrap_add_PATH[@]}"; do
        echo "claptrap_add_PATH[$i]: ${claptrap_add_PATH[$i]}"
      done
    fi
    ;;
esac
