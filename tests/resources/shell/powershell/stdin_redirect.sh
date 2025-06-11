#!/usr/bin/env pwsh

Get-Content $Env:CLAPTRAP_SPEC | & $Env:CLAPTRAP_BIN --spec-format $Env:CLAPTRAP_SPEC_FORMAT --spec - -- $Args | Invoke-Expression

Write-Output "mode: $Env:claptrap_mode"
Write-Output "protocol: $Env:claptrap_protocol"
