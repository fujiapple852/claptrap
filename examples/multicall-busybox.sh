#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec multicall-busybox.toml -- "$@")"

case "${claptrap_subcommand:-}" in
  true)
    exit 0
    ;;
  false)
    exit 1
    ;;
  busybox)
    echo "busybox main command"
    ;;
  *)
    echo "unknown applet"
    ;;
esac
