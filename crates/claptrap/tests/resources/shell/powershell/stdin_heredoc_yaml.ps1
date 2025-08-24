#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$spec = @'
name: myapp
version: 0.1.0
args:
  mode:
    short: m
    long: mode
  protocol:
    short: p
    long: protocol
'@
. ([scriptblock]::Create(( $spec | & $env:CLAPTRAP_BIN --spec - --output-format powershell -- @args ) -join "`n"))

Write-Output "mode: $env:claptrap_mode"
Write-Output "protocol: $env:claptrap_protocol"

