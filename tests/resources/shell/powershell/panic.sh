#!/usr/bin/env pwsh

$envVars = @'
name = "myapp"
[args]
# this wil cause clap to panic
mode = { index = 2 }
'@ | & $Env:CLAPTRAP_BIN --spec - -- $Args | Out-String
Invoke-Expression $envVars
