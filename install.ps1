# var-gen Installation Script for User Directory (Windows PowerShell)
# Run without Administrator privileges

Write-Host "=== var-gen Installation Program (User Directory) ===" -ForegroundColor Green

# Set installation path to user directory
$InstallPath = "$env:LOCALAPPDATA\var-gen"
$BinaryName = "var-gen.exe"

Write-Host "Installation path: $InstallPath" -ForegroundColor Yellow

# Create installation directory (clean old installation)
if (Test-Path $InstallPath) {
    Write-Host "Detected existing installation, uninstalling..." -ForegroundColor Yellow
    Remove-Item -Path $InstallPath -Recurse -Force
}
New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null

# Set source binary paths based on script location
$SourceBinary = Join-Path -Path $PSScriptRoot -ChildPath "..\bin\$BinaryName"
$DevSourceBinary = Join-Path -Path $PSScriptRoot -ChildPath "..\..\target\release\$BinaryName"
$TargetBinary = Join-Path -Path $InstallPath -ChildPath $BinaryName

Write-Host "Checking for binary file..." -ForegroundColor Yellow
Write-Host "Source 1: $SourceBinary" -ForegroundColor Gray
Write-Host "Source 2: $DevSourceBinary" -ForegroundColor Gray

# Check which binary file exists
if (Test-Path -Path $SourceBinary -PathType Leaf) {
    Copy-Item -Path $SourceBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from installation package: $TargetBinary" -ForegroundColor Green
} elseif (Test-Path -Path $DevSourceBinary -PathType Leaf) {
    Copy-Item -Path $DevSourceBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from development build: $TargetBinary" -ForegroundColor Green
} else {
    Write-Host "Cannot find compiled binary file!" -ForegroundColor Red
    Write-Host "Searched locations:" -ForegroundColor Yellow
    Write-Host "  - $SourceBinary" -ForegroundColor Yellow
    Write-Host "  - $DevSourceBinary" -ForegroundColor Yellow
    exit 1
}

# Add to user PATH (not system PATH)
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstallPath*") {
    Write-Host "Adding to user environment variables..." -ForegroundColor Green
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallPath", "User")
    Write-Host "Added $InstallPath to user PATH" -ForegroundColor Green
} else {
    Write-Host "Path already exists in user PATH" -ForegroundColor Yellow
}

# Create Start Menu shortcut
$shortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\var-gen.lnk"
if (Test-Path -Path $shortcutPath) {
    Remove-Item -Path $shortcutPath -Force
}

# Create shortcut using WScript.Shell (more compatible)
try {
    $wshell = New-Object -ComObject WScript.Shell
    $shortcut = $wshell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $TargetBinary
    $shortcut.WorkingDirectory = $InstallPath
    $shortcut.Description = "var-gen - Smart Variable Naming Tool"
    $shortcut.Save()
    Write-Host "Start Menu shortcut created: $shortcutPath" -ForegroundColor Green
} catch {
    Write-Host "Warning: Failed to create Start Menu shortcut" -ForegroundColor Yellow
    Write-Host "Error: $_" -ForegroundColor Gray
}

Write-Host "=== Installation Complete! ===" -ForegroundColor Green
Write-Host "Please reopen Command Prompt or PowerShell to make environment variables effective" -ForegroundColor Yellow
Write-Host "After installation, you can run: var-gen --help" -ForegroundColor Green

# Verify installation
Write-Host "Verifying installation..." -ForegroundColor Green
# Update current session PATH
$env:Path = [Environment]::GetEnvironmentVariable("Path", "User") + ";" + [Environment]::GetEnvironmentVariable("Path", "Machine")
Start-Sleep -Seconds 1

try {
    $Result = & "$TargetBinary" --version 2>&1
    if ($Result) {
        Write-Host "Verification successful! Version info:" -ForegroundColor Green
        Write-Host $Result -ForegroundColor Cyan
    } else {
        Write-Host "Verification warning: No version output" -ForegroundColor Yellow
    }
} catch {
    Write-Host "Verification warning: Cannot execute var-gen" -ForegroundColor Yellow
}

Write-Host "Installation completed successfully!" -ForegroundColor Green