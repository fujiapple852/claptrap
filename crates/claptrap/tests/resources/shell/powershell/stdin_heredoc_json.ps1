#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$spec = @'
{
  "name": "myapp",
  "version": "0.1.0",
  "args": {
    "mode": {
      "long": "mode",
      "short": "m"
    },
    "protocol": {
      "long": "protocol",
      "short": "p"
    }
  }
}
'@
. ([scriptblock]::Create(( $spec | & $env:CLAPTRAP_BIN --spec - --output-format powershell -- @args ) -join "`n"))

Write-Output "mode: $env:claptrap_mode"
Write-Output "protocol: $env:claptrap_protocol"

