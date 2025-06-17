#!/usr/bin/env fish

if not type -q claptrap
    echo "claptrap command not found. Please install it first, see https://claptrap.cli.rs for instructions."
    exit 1
end

eval (claptrap --shell fish --spec spec.toml -- $argv | string collect)
