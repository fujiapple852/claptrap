#!/usr/bin/env pwsh

if (-not (Get-Command claptrap -ErrorAction SilentlyContinue)) {
    Write-Host "claptrap command not found. Please install it first, see https://claptrap.cli.rs for instructions."
    exit 1
}

Invoke-Expression (& claptrap --spec spec.toml -- $Args)
