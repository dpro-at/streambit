param (
    [string]$TargetFolder = "C:\Users\Dpro GmbH\Pictures\Screenshots"
)

# Set encoding AFTER param block
$OutputEncoding = [System.Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$PythonPath = "C:\Users\Dpro GmbH\AppData\Local\Programs\Python\Python312\python.exe"
$ScriptPath = "$PSScriptRoot\streambit-web-ui\benchmark_python.py"

Write-Host "NOTE: Using Target Folder: $TargetFolder" -ForegroundColor DarkGray

Write-Host "`n[START] StreamBit vs Python Benchmark Comparison" -ForegroundColor Cyan
Write-Host "=============================================" -ForegroundColor Gray

if (-not (Test-Path $TargetFolder)) {
    Write-Error "Folder not found: $TargetFolder"
    exit 1
}

# ---------------------------------------------------------
# 1. Run Python Benchmark
# ---------------------------------------------------------
Write-Host "`n[PYTHON] Running Pillow Benchmark (might take a moment)..." -ForegroundColor Yellow

# Get files
$files = Get-ChildItem $TargetFolder -File | 
         Where-Object { $_.Extension -match "\.(jpg|jpeg|png|webp|bmp|gif)$" } | 
         Select-Object -ExpandProperty FullName

$fileCount = $files.Count
if ($fileCount -eq 0) {
    Write-Error "No images found in folder."
    exit 1
}
Write-Host "   Found $fileCount images." -ForegroundColor DarkGray

# Save list to temp JSON file to avoid pipe encoding issues
$tempJsonFile = [System.IO.Path]::GetTempFileName()
$files | ConvertTo-Json -Compress | Set-Content -Path $tempJsonFile -Encoding UTF8

# Run Python script
# Redirect stderr to $null to avoid polluting the JSON output
$pythonOutput = & $PythonPath $ScriptPath --json-file $tempJsonFile 2>$null | Out-String

# Clean up temp file
if (Test-Path $tempJsonFile) { Remove-Item $tempJsonFile }

$pyTime = 0
$pyThroughput = 0

try {
    # Parse JSON output
    $pyResult = $pythonOutput | ConvertFrom-Json
    
    if ($pyResult.error) {
        Write-Error "Python Error: $($pyResult.error)"
        exit 1
    }

    $pyTime = $pyResult.time_ms
    $pyThroughput = $pyResult.throughput
    
    Write-Host "   Done!" -ForegroundColor Green
} catch {
    Write-Error "Failed to parse Python output. Raw output below:"
    Write-Host $pythonOutput
    exit 1
}

# ---------------------------------------------------------
# 2. Run Rust Benchmark
# ---------------------------------------------------------
Write-Host "`n[RUST] Running StreamBit Benchmark..." -ForegroundColor Red

# Define cargo path explicitely to avoid PATH issues in script scope
$CargoPath = "C:\Users\Dpro GmbH\.cargo\bin\cargo.exe"

# check if cargo exists, if not try just "cargo"
if (-not (Test-Path $CargoPath)) { $CargoPath = "cargo" }

# Build first to ensure release binary is ready (quietly)
& $CargoPath build --release -p streambit-cli --quiet

# Run CLI and capture output
$rustOutputRaw = & $CargoPath run --release -p streambit-cli --quiet -- process-folder $TargetFolder 2>&1 | Out-String

$rustTime = 0
$rustThroughput = 0

# Parse Rust output using Regex
if ($rustOutputRaw -match "Time:\s+([\d\.]+)\s+ms") {
    $rustTime = [double]$Matches[1]
} else {
    Write-Error "Could not parse Rust Time. Output:`n$rustOutputRaw"
    exit 1
}

if ($rustOutputRaw -match "Throughput:\s+([\d\.]+)\s+images/sec") {
    $rustThroughput = [double]$Matches[1]
} else {
    Write-Error "Could not parse Rust Throughput."
    exit 1
}

Write-Host "   Done!" -ForegroundColor Green

# ---------------------------------------------------------
# 3. Display Comparison
# ---------------------------------------------------------
$speedup = 0
if ($rustTime -gt 0) {
    $speedup = $pyTime / $rustTime
}

Write-Host "`n[RESULTS] Final Comparison" -ForegroundColor Cyan
Write-Host "========================" -ForegroundColor Gray

# Python Results
Write-Host "`n[PYTHON] Pillow:" -ForegroundColor Yellow
Write-Host "   Time:       " -NoNewline
Write-Host "$("{0:N2}" -f $pyTime) ms" -ForegroundColor White
Write-Host "   Throughput: " -NoNewline
Write-Host "$("{0:N0}" -f $pyThroughput) images/sec" -ForegroundColor White

# Rust Results
Write-Host "`n[RUST] StreamBit:" -ForegroundColor Red
Write-Host "   Time:       " -NoNewline
Write-Host "$("{0:N2}" -f $rustTime) ms" -ForegroundColor White
Write-Host "   Throughput: " -NoNewline
Write-Host "$("{0:N0}" -f $rustThroughput) images/sec" -ForegroundColor White

# Winner
Write-Host "`n[WINNER] Conclusion:" -ForegroundColor Green
Write-Host "   StreamBit is " -NoNewline
Write-Host "$("{0:N2}" -f $speedup)x faster" -ForegroundColor Yellow -NoNewline
Write-Host " than Python!" -ForegroundColor Green
Write-Host ""
