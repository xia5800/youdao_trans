# Build 优道翻译 Portable Directory
# Usage: .\build-portable.ps1

$ErrorActionPreference = "Stop"

$PROJECT_NAME = "YoudaoTranslate"
$OUTPUT_DIR = Join-Path $PSScriptRoot "dist-portable\$PROJECT_NAME"

# Step 1: Build frontend
Write-Host "=== Step 1/3: Building frontend (npm run build)" -ForegroundColor Cyan
Push-Location $PSScriptRoot
npm run build
if (-not $?) { throw "Frontend build failed" }

# Step 2: Build Rust app (via tauri build to properly embed frontend assets)
Write-Host "=== Step 2/3: Building Rust app (npx tauri build --no-bundle)" -ForegroundColor Cyan
Push-Location "src-tauri"
try {
    npx tauri build --no-bundle
    if (-not $?) { throw "Rust build failed" }
} finally {
    Pop-Location
}

# Step 3: Verify app.exe exists
$appExe = Join-Path $PSScriptRoot "src-tauri\target\release\app.exe"
if (-not (Test-Path -LiteralPath $appExe)) {
    Write-Error "app.exe not found. Build may have failed."
    exit 1
}
Write-Host "app.exe size: $([math]::Round((Get-Item $appExe).Length / 1MB, 2)) MB" -ForegroundColor Green

# Step 4: Create portable directory structure
Write-Host "=== Step 3/3: Assembling portable directory" -ForegroundColor Cyan
Write-Host "  Output: $OUTPUT_DIR" -ForegroundColor Yellow

# Clean and recreate directories
if (Test-Path -LiteralPath $OUTPUT_DIR) {
    Remove-Item -LiteralPath $OUTPUT_DIR -Recurse -Force
}
New-Item -ItemType Directory -Path $OUTPUT_DIR -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $OUTPUT_DIR "models\dict") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $OUTPUT_DIR "models\ocr") -Force | Out-Null

# Copy files
Copy-Item -LiteralPath $appExe -Destination (Join-Path $OUTPUT_DIR "app.exe") -Force

$dbFile = Join-Path $PSScriptRoot "models\dict\ecdict.db"
if (Test-Path -LiteralPath $dbFile) {
    Copy-Item -LiteralPath $dbFile -Destination (Join-Path $OUTPUT_DIR "models\dict\ecdict.db") -Force
    $dbSize = [math]::Round((Get-Item $dbFile).Length / 1MB, 2)
    Write-Host "  Copied ecdict.db ($dbSize MB)" -ForegroundColor Green
} else {
    Write-Warning "ecdict.db not found at $dbFile — skipping"
}

$ocrDir = Join-Path $PSScriptRoot "models\ocr"
if (Test-Path -LiteralPath $ocrDir) {
    Copy-Item -LiteralPath "$ocrDir\*" -Destination (Join-Path $OUTPUT_DIR "models\ocr") -Recurse -Force
    Write-Host "  Copied OCR models" -ForegroundColor Green
} else {
    Write-Warning "models/ocr not found — skipping"
}

# Step 5: Show result
$totalSize = [math]::Round((Get-ChildItem -LiteralPath $OUTPUT_DIR -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB, 2)
Write-Host "=== Done! Portable directory created:" -ForegroundColor Green
Write-Host "  $OUTPUT_DIR" -ForegroundColor Yellow
Write-Host "  Total size: $totalSize MB" -ForegroundColor Yellow
Write-Host ""
Write-Host "  Contents:" -ForegroundColor Cyan
Get-ChildItem -LiteralPath $OUTPUT_DIR -Recurse | ForEach-Object {
    $relative = $_.FullName.Substring($OUTPUT_DIR.Length + 1)
    if (-not $_.PSIsContainer) {
        $size = [math]::Round($_.Length / 1KB, 1)
        Write-Host "    $relative  ($size KB)" -ForegroundColor Gray
    } else {
        Write-Host "    $relative\" -ForegroundColor DarkGray
    }
}
