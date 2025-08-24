#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

. ([scriptblock]::Create(( & $env:CLAPTRAP_BIN --spec $env:CLAPTRAP_SPEC --output-format powershell -- @args ) -join "`n"))

Write-Output "mode: $env:claptrap_mode"
Write-Output "protocol: $env:claptrap_protocol"