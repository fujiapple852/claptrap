#!/usr/bin/env bash

set -euo pipefail

eval "$(docker run -it -v "${PWD}":/data fujiapple/claptrap:latest --spec /data/spec.toml -- "$@")"

# shellcheck disable=SC2154
echo "claptrap__subcommand: ${claptrap__subcommand}"
case "${claptrap__subcommand}" in
  "run")
    if [ -n "${claptrap_run_interactive+x}" ]; then
      echo "claptrap_run_interactive: ${claptrap_run_interactive}"
    fi
    if [ -n "${claptrap_run_tty+x}" ]; then
      echo "claptrap_run_tty: ${claptrap_run_tty}"
    fi
    if [ -n "${claptrap_run_volume+x}" ]; then
      echo "claptrap_run_volume: ${claptrap_run_volume}"
    fi
    if [ -n "${claptrap_run_image+x}" ]; then
      echo "claptrap_run_image: ${claptrap_run_image}"
    fi
    ;;
esac