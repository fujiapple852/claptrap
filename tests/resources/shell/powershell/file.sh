#!/usr/bin/env pwsh

$envVars = & $Env:CLAPTRAP_BIN --spec $Env:CLAPTRAP_SPEC -- $Args
Invoke-Expression $envVars

Write-Output "mode: $Env:claptrap_mode"
Write-Output "protocol: $Env:claptrap_protocol"
