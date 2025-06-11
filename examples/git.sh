#!/usr/bin/env bash

set -euo pipefail

# parse arguments using claptrap
eval "$(claptrap --spec git.toml -- "$@")"

case "${claptrap_subcommand:-}" in
  clone)
    echo "Cloning $claptrap_REMOTE"
    ;;
  diff)
    echo "Diffing ${claptrap_base:-stage}..${claptrap_head:-worktree} ${claptrap_path:-} (color=${claptrap_color:-auto})"
    ;;
  push)
    echo "Pushing to $claptrap_REMOTE"
    ;;
  add)
    echo "Adding: ${claptrap_PATH[*]}"
    ;;
  stash)
    case "${claptrap_subcommand_stash:-push}" in
      push)
        echo "Stash push message: ${claptrap_message:-}"
        ;;
      pop)
        echo "Stash pop ${claptrap_STASH:-}"
        ;;
      apply)
        echo "Stash apply ${claptrap_STASH:-}"
        ;;
    esac
    ;;
  *)
    if [[ -n "${claptrap_subcommand:-}" ]]; then
      echo "External command: ${claptrap_subcommand}"
    fi
    ;;
esac
