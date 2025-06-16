#!/usr/bin/env fish

if not type -q claptrap
    echo "claptrap command not found. Please install it first, see https://claptrap.cli.rs for instructions."
    exit 1
end

claptrap --spec spec.toml -- $argv | source
