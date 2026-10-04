$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
$qualia = Join-Path $repo '..\qualiaDB'
$expected = (Get-Content -LiteralPath (Join-Path $repo 'qualia.ref') -Raw).Trim()
$actual = (git -c safe.directory=C:/github/qualiaDB -C $qualia rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0 -or $actual -ne $expected) {
    throw "QualiaDB checkout must be at $expected (found $actual)"
}
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
