$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
Push-Location $repo
try {
    wasm-pack build crates/rolling-commons-shell --target web --out-dir ../../web/pkg --release
    if ($LASTEXITCODE -ne 0) { throw 'wasm-pack failed' }
    $pkgIgnore = Join-Path $repo 'web/pkg/.gitignore'
    if (Test-Path -LiteralPath $pkgIgnore) { Remove-Item -LiteralPath $pkgIgnore }
    node scripts/export-game-assets.mjs
    if ($LASTEXITCODE -ne 0) { throw 'asset export failed' }
} finally {
    Pop-Location
}
