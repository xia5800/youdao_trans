# Build 优道翻译 Release Installer
# Usage: .\build-installer.ps1

$ErrorActionPreference = "Stop"

# Step 1: Ensure NSIS available
$makensis = "C:\Program Files (x86)\NSIS\Bin\makensis.exe"
if (-not (Test-Path -LiteralPath $makensis)) {
    Write-Error "NSIS not found at $makensis. Please install NSIS first."
    exit 1
}

# Step 2: Build frontend
Write-Host "=== Step 1/5: Building frontend (npm run build)" -ForegroundColor Cyan
Push-Location $PSScriptRoot
npm run build
if (-not $?) { throw "Frontend build failed" }

# Step 3: Build Rust app (via tauri build to properly embed frontend assets)
Write-Host "=== Step 2/5: Building Rust app (npx tauri build --bundles none)" -ForegroundColor Cyan
Push-Location "src-tauri"
try {
    npx tauri build --no-bundle
    if (-not $?) { throw "Rust build failed" }
} finally {
    Pop-Location
}

# Step 4: Verify artifacts
Write-Host "=== Step 3/5: Verifying artifacts" -ForegroundColor Cyan
$appExe = Join-Path $PSScriptRoot "src-tauri\target\release\app.exe"
if (-not (Test-Path -LiteralPath $appExe)) {
    Write-Error "app.exe not found. Build may have failed."
    exit 1
}
Write-Host "  app.exe: $([math]::Round((Get-Item $appExe).Length / 1MB, 2)) MB" -ForegroundColor Green

$dbFile = Join-Path $PSScriptRoot "models\dict\ecdict.db"
if (-not (Test-Path -LiteralPath $dbFile)) {
    Write-Error "ecdict.db not found at $dbFile"
    exit 1
}
$dbSize = [math]::Round((Get-Item $dbFile).Length / 1MB, 2)
Write-Host "  ecdict.db: $dbSize MB" -ForegroundColor Green

$ocrDir = Join-Path $PSScriptRoot "models\ocr\PaddleOCR"
$ocrFiles = @("PP-OCRv6_medium_det.onnx", "PP-OCRv6_medium_rec.onnx", "ppocrv6_dict.txt")
foreach ($file in $ocrFiles) {
    $ocrFile = Join-Path $ocrDir $file
    if (-not (Test-Path -LiteralPath $ocrFile)) {
        Write-Error "$file not found at $ocrFile"
        exit 1
    }
    Write-Host "  $file : $([math]::Round((Get-Item $ocrFile).Length / 1MB, 2)) MB" -ForegroundColor Green
}

# Step 5: Create output directory
$outDir = Join-Path $PSScriptRoot "dist-installer"
New-Item -ItemType Directory -Path $outDir -Force | Out-Null

# Step 6: Compile NSIS installer
Write-Host "=== Step 4/5: Compiling NSIS installer" -ForegroundColor Cyan
Push-Location "src-tauri\windows"
try {
    & $makensis "installer.nsi"
    if (-not $?) { throw "NSIS compilation failed" }
} finally {
    Pop-Location
}

# Step 7: Show result
Write-Host "=== Step 5/5: Result" -ForegroundColor Cyan
$installer = Get-ChildItem -Path $outDir -Filter "*_setup.exe" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
if ($installer) {
    Write-Host "=== Done! Installer created:" -ForegroundColor Green
    Write-Host "  $($installer.FullName)" -ForegroundColor Yellow
    Write-Host "  Size: $([math]::Round($installer.Length / 1MB, 2)) MB" -ForegroundColor Yellow
} else {
    Write-Error "Installer not found in $outDir"
}

Pop-Location
