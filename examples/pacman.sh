#!/usr/bin/env bash

set -euo pipefail

eval "$(claptrap --spec pacman.toml -- "$@")"

case "${claptrap_subcommand:-}" in
  query)
    if [[ -n "${claptrap_search[*]:-}" ]]; then
      echo "Searching Locally for ${claptrap_search[*]}..."
    elif [[ -n "${claptrap_info[*]:-}" ]]; then
      echo "Retrieving info for ${claptrap_info[*]}..."
    else
      echo "Displaying all locally installed packages..."
    fi
    ;;
  sync)
    if [[ -n "${claptrap_search[*]:-}" ]]; then
      echo "Searching for ${claptrap_search[*]}..."
    else
      if [[ "${claptrap_info:-false}" == "true" ]]; then
        echo "Retrieving info for ${claptrap_package[*]}..."
      else
        echo "Installing ${claptrap_package[*]}..."
      fi
    fi
    ;;
  *)
    ;;
esac
