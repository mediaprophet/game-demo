# Maslows Challenge — local CI commands (Windows host).
# Usage: scripts\ci.ps1 [-Step all|fmt|check|test|wasm|assets]
param([string]$Step = "all")

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root

function Fmt {
    cargo fmt -p rolling-commons-shell -- --check
}

function Check {
    # Full QualiaDB WASM profile, including render and inference compilation.
    cargo check --workspace --target wasm32-unknown-unknown --locked
    if ($LASTEXITCODE -ne 0) { throw 'WASM check failed' }
}

function Tests {
    cargo test --workspace --target wasm32-unknown-unknown --no-run --locked
    if ($LASTEXITCODE -ne 0) { throw 'WASM test compile failed' }
}

function Wasm {
    & (Join-Path $PSScriptRoot 'build-game.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'game build failed' }
}

function Assets {
    foreach ($file in @('assets/generated/manifest.json', 'web/assets/maslows-challenge-scenes.hmc', 'web/pkg/rolling_commons_shell_bg.wasm')) {
        $path = Join-Path $root $file
        if (-not (Test-Path -LiteralPath $path) -or (Get-Item -LiteralPath $path).Length -eq 0) {
            throw "Missing game build output: $file"
        }
    }
    Write-Host 'Qualia game assets and HMC present'
}

switch ($Step) {
    "fmt"    { Fmt }
    "check"  { Check }
    "test"   { Tests }
    "wasm"   { Wasm }
    "assets" { Assets }
    "all"    { Fmt; Check; Tests; Wasm; Assets }
    default  { Write-Error "unknown step: $Step" }
}

Pop-Location
