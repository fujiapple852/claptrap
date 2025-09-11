#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$spec = @'
name = "myapp"
[args]
# this wil cause clap to panic
mode = { index = 2 }
'@
. ([scriptblock]::Create(( $spec | & $env:CLAPTRAP_BIN --spec - --output-format powershell -- @args ) -join "`n"))