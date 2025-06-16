#!/usr/bin/env pwsh

$envVars = @'
name = "myapp"
version = "0.1.0"
[args]
mode = { short = "m", long = "mode" }
protocol = { short = "p", long = "protocol" }
'@ | & $Env:CLAPTRAP_BIN --spec - -- $Args | Out-String
Invoke-Expression $envVars

Write-Output "mode: $Env:claptrap_mode"
Write-Output "protocol: $Env:claptrap_protocol"
