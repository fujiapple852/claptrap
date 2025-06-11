#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec multicall-hostname.toml -- "$@")"

case "${claptrap_subcommand:-}" in
  hostname)
    echo "www"
    ;;
  dnsdomainname)
    echo "example.com"
    ;;
  *)
    echo "unknown applet"
    ;;
esac
