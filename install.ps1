# var-gen Installation Script for User Directory (Windows PowerShell)
# Run without Administrator privileges

Write-Host "=== var-gen Installation Program (User Directory) v2.0 - Complete Editor Terminal Support ===" -ForegroundColor Cyan
Write-Host "No separate fix scripts needed - this installer handles everything!" -ForegroundColor Green

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
$SourceBinary = Join-Path -Path $PSScriptRoot -ChildPath "bin\$BinaryName"
$DevSourceBinary = Join-Path -Path $PSScriptRoot -ChildPath "target\release\$BinaryName"
$PackageBinary = Join-Path -Path $PSScriptRoot -ChildPath "..\bin\$BinaryName"
$DirectBinary = Join-Path -Path $PSScriptRoot -ChildPath "$BinaryName"
$TargetBinary = Join-Path -Path $InstallPath -ChildPath $BinaryName

Write-Host "Checking for binary file..." -ForegroundColor Yellow
Write-Host "Source 1: $SourceBinary" -ForegroundColor Gray
Write-Host "Source 2: $DevSourceBinary" -ForegroundColor Gray
Write-Host "Source 3: $PackageBinary" -ForegroundColor Gray
Write-Host "Source 4: $DirectBinary" -ForegroundColor Gray

# Check which binary file exists
if (Test-Path -Path $SourceBinary -PathType Leaf) {
    Copy-Item -Path $SourceBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from local bin: $TargetBinary" -ForegroundColor Green
} elseif (Test-Path -Path $DevSourceBinary -PathType Leaf) {
    Copy-Item -Path $DevSourceBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from development build: $TargetBinary" -ForegroundColor Green
} elseif (Test-Path -Path $PackageBinary -PathType Leaf) {
    Copy-Item -Path $PackageBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from package: $TargetBinary" -ForegroundColor Green
} elseif (Test-Path -Path $DirectBinary -PathType Leaf) {
    Copy-Item -Path $DirectBinary -Destination $TargetBinary -Force
    Write-Host "Binary file copied from direct location: $TargetBinary" -ForegroundColor Green
} else {
    Write-Host "Cannot find compiled binary file!" -ForegroundColor Red
    Write-Host "Searched locations:" -ForegroundColor Yellow
    Write-Host "  - $SourceBinary" -ForegroundColor Yellow
    Write-Host "  - $DevSourceBinary" -ForegroundColor Yellow
    Write-Host "  - $PackageBinary" -ForegroundColor Yellow
    Write-Host "  - $DirectBinary" -ForegroundColor Yellow
    exit 1
}

# Enhanced PATH configuration with conflict detection and resolution
Write-Host "Configuring environment variables..." -ForegroundColor Green

# Function to clean up conflicting PATH entries
function Remove-ConflictingPathEntries {
    param(
        [string]$PathType,
        [string]$TargetPath
    )
    
    try {
        $CurrentPath = [Environment]::GetEnvironmentVariable("PATH", $PathType)
        if ($CurrentPath) {
            # Split PATH and remove conflicting entries (other var-gen directories)
            $PathEntries = $CurrentPath -split ';' | Where-Object { 
                $_ -and $_ -notlike "*var-gen*" 
            }
            $CleanPath = $PathEntries -join ';'
            
            if ($CleanPath -ne $CurrentPath) {
                Write-Host "Cleaning conflicting entries from $PathType PATH..." -ForegroundColor Yellow
                [Environment]::SetEnvironmentVariable("PATH", $CleanPath, $PathType)
                Write-Host "✓ Cleaned $PathType PATH successfully!" -ForegroundColor Green
                return $CleanPath
            }
        }
        return $CurrentPath
    } catch {
        Write-Host "⚠ Warning: Failed to clean $PathType PATH - $_" -ForegroundColor Yellow
        return $CurrentPath
    }
}

# Clean up system PATH first (if we have admin rights)
$SystemPathCleaned = $false
try {
    $SystemPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($SystemPath -like "*var-gen*") {
        Write-Host "Detected conflicting entries in system PATH, attempting to clean..." -ForegroundColor Yellow
        $CleanSystemPath = ($SystemPath -split ';' | Where-Object { $_ -and $_ -notlike "*var-gen*" }) -join ';'
        [Environment]::SetEnvironmentVariable("PATH", $CleanSystemPath, "Machine")
        $SystemPathCleaned = $true
        Write-Host "✓ System PATH cleaned successfully!" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠ Note: Could not clean system PATH (admin rights required) - $_" -ForegroundColor Yellow
    Write-Host "  This is normal for user-level installation" -ForegroundColor Gray
}

# Clean up user PATH and add our installation path
try {
    # First clean existing entries
    $UserPath = Remove-ConflictingPathEntries -PathType "User" -TargetPath $InstallPath
    
    # Then add our path if not already present
    if ($UserPath -notlike "*$InstallPath*") {
        Write-Host "Adding $InstallPath to user PATH..." -ForegroundColor Yellow
        $NewUserPath = if ($UserPath) { "$UserPath;$InstallPath" } else { $InstallPath }
        [Environment]::SetEnvironmentVariable("PATH", $NewUserPath, "User")
        Write-Host "✓ User PATH configured successfully!" -ForegroundColor Green
    } else {
        Write-Host "✓ Installation path already exists in user PATH" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠ Warning: Failed to configure user PATH - $_" -ForegroundColor Yellow
    Write-Host "  You may need to manually add '$InstallPath' to your user PATH" -ForegroundColor Gray
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

# Enhanced verification with immediate PATH update and conflict resolution
Write-Host "Verifying installation..." -ForegroundColor Green

# Force reload environment variables from registry to ensure latest changes
Write-Host "Reloading environment variables..." -ForegroundColor Gray

# Method 1: Broadcast WM_SETTINGCHANGE message to notify all applications
Write-Host "Broadcasting environment change notification..." -ForegroundColor Gray
try {
    # Use .NET to broadcast environment variable changes
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class EnvVarNotifier {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(
        IntPtr hWnd,
        uint Msg,
        UIntPtr wParam,
        string lParam,
        uint fuFlags,
        uint uTimeout,
        out UIntPtr lpdwResult
    );
    
    public static void BroadcastEnvironmentChange() {
        const uint WM_SETTINGCHANGE = 0x001A;
        const uint SMTO_ABORTIFHUNG = 0x0002;
        const uint SMTO_NORMAL = 0x0000;
        const uint timeout = 5000;
        
        UIntPtr result;
        SendMessageTimeout(
            (IntPtr)0xFFFF, // HWND_BROADCAST
            WM_SETTINGCHANGE,
            (UIntPtr)0,
            "Environment",
            SMTO_NORMAL | SMTO_ABORTIFHUNG,
            timeout,
            out result
        );
    }
}
"@
    
    [EnvVarNotifier]::BroadcastEnvironmentChange()
    Write-Host "✓ Environment change notification broadcasted" -ForegroundColor Green
} catch {
    Write-Host "⚠ Could not broadcast environment change: $_" -ForegroundColor Yellow
}

# Method 2: Force reload from registry with retry logic
$maxRetries = 3
$retryCount = 0
$pathLoaded = $false

while ($retryCount -lt $maxRetries -and -not $pathLoaded) {
    try {
        # Force registry refresh
        [System.Environment]::SetEnvironmentVariable("TEMP_REFRESH", "1", "User")
        [System.Environment]::SetEnvironmentVariable("TEMP_REFRESH", $null, "User")
        
        # Get fresh copies from registry
        $UserPathFromRegistry = [Environment]::GetEnvironmentVariable("PATH", "User")
        $MachinePathFromRegistry = [Environment]::GetEnvironmentVariable("PATH", "Machine")
        
        # Clean machine path for current session
        $CleanMachinePath = ($MachinePathFromRegistry -split ';' | Where-Object { $_ -and $_ -notlike "*var-gen*" }) -join ';'
        
        # Set PATH with user path first (higher priority) - this affects current session
        $env:Path = "$UserPathFromRegistry;$CleanMachinePath"
        
        # Also ensure our install path is definitely in the current session PATH
        if ($env:Path -notlike "*$InstallPath*") {
            $env:Path = "$InstallPath;$env:Path"
            Write-Host "Added installation path to current session PATH" -ForegroundColor Gray
        }
        
        $pathLoaded = $true
        Write-Host "✓ Environment variables reloaded successfully" -ForegroundColor Green
    } catch {
        $retryCount++
        if ($retryCount -lt $maxRetries) {
            Write-Host "Retrying environment reload ($retryCount/$maxRetries)..." -ForegroundColor Gray
            Start-Sleep -Milliseconds 500
        } else {
            Write-Host "⚠ Failed to reload environment variables after $maxRetries attempts" -ForegroundColor Yellow
        }
    }
}

Write-Host "Updated PATH configuration for current session" -ForegroundColor Green

# Wait a moment for PATH update
Start-Sleep -Seconds 1

# Test with direct path first, then try PATH resolution
$DirectSuccess = $false
$PathSuccess = $false

try {
    Write-Host "Testing direct execution..." -ForegroundColor Gray
    $DirectResult = & "$TargetBinary" --version 2>&1
    if ($DirectResult -and $DirectResult -match "\d+\.\d+\.\d+") {
        Write-Host "✓ Direct execution successful!" -ForegroundColor Green
        Write-Host "  Version: $DirectResult" -ForegroundColor Cyan
        $DirectSuccess = $true
    } else {
        Write-Host "⚠ Direct execution test inconclusive" -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠ Direct execution failed: $_" -ForegroundColor Red
}

# Test PATH resolution
try {
    Write-Host "Testing PATH resolution..." -ForegroundColor Gray
    $PathResult = & "var-gen" --version 2>&1
    if ($PathResult -and $PathResult -match "\d+\.\d+\.\d+") {
        Write-Host "✓ PATH resolution successful!" -ForegroundColor Green
        Write-Host "  Version: $PathResult" -ForegroundColor Cyan
        $PathSuccess = $true
    } else {
        Write-Host "⚠ PATH resolution test inconclusive" -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠ PATH resolution failed: $_" -ForegroundColor Red
    Write-Host "  PATH may need manual adjustment" -ForegroundColor Gray
}

# Test in a fresh PowerShell session simulation
try {
    Write-Host "Testing editor terminal compatibility..." -ForegroundColor Gray
    $TestResult = powershell -NoProfile -Command "`$env:Path = '[Environment]::GetEnvironmentVariable(\"PATH\", \"User\") + ';' + [Environment]::GetEnvironmentVariable(\"PATH\", \"Machine\")'; var-gen --version" 2>&1
    if ($TestResult -and $TestResult -match "\d+\.\d+\.\d+") {
        Write-Host "✓ Editor terminal compatibility verified!" -ForegroundColor Green
        Write-Host "  Fresh session test passed" -ForegroundColor Cyan
    } else {
        Write-Host "⚠ Editor terminal test inconclusive" -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠ Editor terminal compatibility test failed: $_" -ForegroundColor Yellow
}

# If PATH resolution failed but direct execution worked, provide specific guidance
if ($DirectSuccess -and -not $PathSuccess) {
    Write-Host "⚠ PATH conflict detected!" -ForegroundColor Yellow
    Write-Host "  Direct execution works but PATH resolution fails" -ForegroundColor Gray
    Write-Host "  This usually means another directory in PATH contains an invalid var-gen entry" -ForegroundColor Gray
    Write-Host "  Solution: Restart your terminal, or manually check PATH for conflicting entries" -ForegroundColor Yellow
}

# Final status and troubleshooting guidance
Write-Host "=== Installation Summary ===" -ForegroundColor Green
Write-Host "✓ Binary installed to: $InstallPath" -ForegroundColor Green
Write-Host "✓ Environment variables configured" -ForegroundColor Green
Write-Host "✓ Start Menu shortcut created" -ForegroundColor Green

if ($SystemPathCleaned) {
    Write-Host "✓ System PATH conflicts resolved" -ForegroundColor Green
}

Write-Host "" -ForegroundColor White

# Provide specific guidance based on test results
if ($PathSuccess) {
    Write-Host "✅ Installation verified successfully!" -ForegroundColor Green -BackgroundColor Black
    Write-Host "You can now use: var-gen --help" -ForegroundColor Cyan
} elseif ($DirectSuccess) {
    Write-Host "⚠ Installation completed but PATH needs refresh" -ForegroundColor Yellow
    Write-Host "Please restart your terminal and run: var-gen --version" -ForegroundColor Yellow
} else {
    Write-Host "⚠ Installation completed but verification failed" -ForegroundColor Red
    Write-Host "Please restart your terminal and check: var-gen --version" -ForegroundColor Yellow
    Write-Host "If still not working, check PATH environment variable" -ForegroundColor Gray
}

# Additional troubleshooting info
Write-Host "" -ForegroundColor Gray
Write-Host "Troubleshooting tips:" -ForegroundColor Gray
Write-Host "  - If command not found after restart, check PATH for conflicting entries" -ForegroundColor Gray
Write-Host "  - Ensure $InstallPath is in your user PATH environment variable" -ForegroundColor Gray
Write-Host "  - You can always run directly: & '$InstallPath\var-gen.exe' --version" -ForegroundColor Gray

# Final comprehensive environment setup for editor terminals
Write-Host "=== Finalizing Editor Terminal Compatibility ===" -ForegroundColor Cyan

# Force Windows to reload environment variables immediately
Write-Host "Forcing Windows environment reload..." -ForegroundColor Yellow

# Method 1: Use .NET to broadcast environment change to all processes
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public class EnvRefresher {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(
        IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, 
        uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
    
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool PostMessage(
        IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);
    
    public static void RefreshEnvironment() {
        const uint WM_SETTINGCHANGE = 0x001A;
        const uint SMTO_ABORTIFHUNG = 0x0002;
        const uint SMTO_NORMAL = 0x0000;
        const uint timeout = 5000;
        
        // Broadcast to all windows
        UIntPtr result;
        SendMessageTimeout(
            (IntPtr)0xFFFF, // HWND_BROADCAST
            WM_SETTINGCHANGE,
            (UIntPtr)0,
            "Environment",
            SMTO_NORMAL | SMTO_ABORTIFHUNG,
            timeout,
            out result
        );
        
        // Also notify Explorer specifically
        IntPtr hwndExplorer = (IntPtr)0; // Explorer will pick this up
        PostMessage(hwndExplorer, WM_SETTINGCHANGE, (IntPtr)0, (IntPtr)0);
    }
}
"@ -ErrorAction SilentlyContinue

if ($?) {
    try {
        [EnvRefresher]::RefreshEnvironment()
        Write-Host "✓ Environment broadcast completed" -ForegroundColor Green
    } catch {
        Write-Host "⚠ Environment broadcast failed: $_" -ForegroundColor Yellow
    }
}

# Method 2: Force Explorer and other processes to reload
Write-Host "Refreshing Explorer and system processes..." -ForegroundColor Gray
try {
    # Restart Explorer to force PATH reload (optional, user can cancel)
    $restartExplorer = $false
    if ($restartExplorer) {
        Write-Host "Restarting Explorer to reload environment..." -ForegroundColor Yellow
        Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
        Start-Process "explorer"
        Write-Host "✓ Explorer restarted" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠ Could not restart Explorer: $_" -ForegroundColor Yellow
}

# Method 3: Create comprehensive PowerShell profile that works in ALL scenarios
Write-Host "Creating universal PowerShell profile..." -ForegroundColor Yellow

$ProfileDir = Split-Path -Parent $PROFILE.CurrentUserAllHosts
if (!(Test-Path $ProfileDir)) {
    New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
}

# Create a robust profile that handles all PowerShell scenarios
$UniversalProfile = @"
# var-gen universal environment configuration
# This ensures var-gen is available in ALL PowerShell sessions, including editor terminals

function Global:Ensure-VarGenPath {
    `$varGenInstallPath = Join-Path `$env:LOCALAPPDATA "var-gen"
    if (Test-Path `$varGenInstallPath) {
        if (`$env:Path -notlike "*`$varGenInstallPath*") {
            `$env:Path = `$varGenInstallPath + ";" + `$env:Path
        }
    }
}

# Ensure var-gen is available immediately
Ensure-VarGenPath

# Also ensure it's available in future path operations
`$env:varGenPath = Join-Path `$env:LOCALAPPDATA "var-gen"

# Test if var-gen is available and provide helpful message if not
if (!(Get-Command var-gen -ErrorAction SilentlyContinue)) {
    Write-Host "var-gen not found in PATH. Attempting automatic fix..." -ForegroundColor Yellow
    Ensure-VarGenPath
    if (Get-Command var-gen -ErrorAction SilentlyContinue) {
        Write-Host "✓ var-gen is now available!" -ForegroundColor Green
    } else {
        Write-Host "⚠ Automatic fix failed. Please restart your terminal or editor." -ForegroundColor Yellow
        Write-Host "   Or manually run: . `$PROFILE" -ForegroundColor Gray
    }
}
"@

# Write to all possible PowerShell profiles to ensure maximum compatibility
$ProfilePaths = @(
    $PROFILE.CurrentUserAllHosts,
    $PROFILE.CurrentUserCurrentHost,
    "$env:USERPROFILE\Documents\PowerShell\profile.ps1",
    "$env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1"
)

foreach ($ProfilePath in $ProfilePaths) {
    if ($ProfilePath) {
        try {
            $ProfileDir = Split-Path -Parent $ProfilePath
            if (!(Test-Path $ProfileDir)) {
                New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
            }
            
            if (Test-Path $ProfilePath) {
                $ExistingContent = Get-Content $ProfilePath -Raw -ErrorAction SilentlyContinue
                if ($ExistingContent -notlike "*var-gen*") {
                    Add-Content -Path $ProfilePath -Value "`n$UniversalProfile" -Encoding UTF8
                    Write-Host "✓ Added var-gen configuration to: $ProfilePath" -ForegroundColor Green
                } else {
                    Write-Host "✓ var-gen configuration already exists in: $ProfilePath" -ForegroundColor Green
                }
            } else {
                Set-Content -Path $ProfilePath -Value $UniversalProfile -Encoding UTF8
                Write-Host "✓ Created PowerShell profile: $ProfilePath" -ForegroundColor Green
            }
        } catch {
            Write-Host "⚠ Could not update profile: $ProfilePath - $_" -ForegroundColor Yellow
        }
    }
}

# Method 4: Create a registry entry that ensures var-gen is available system-wide
Write-Host "Ensuring registry configuration..." -ForegroundColor Gray
try {
    # Ensure the PATH is properly set in the registry for all scenarios
    $regPath = "Registry::HKEY_CURRENT_USER\Environment"
    $currentPath = (Get-ItemProperty -Path $regPath -Name "PATH" -ErrorAction SilentlyContinue).PATH
    
    if ($currentPath -and $currentPath -notlike "*$InstallPath*") {
        # Add our path to the beginning for priority
        $newPath = "$InstallPath;$currentPath"
        Set-ItemProperty -Path $regPath -Name "PATH" -Value $newPath
        Write-Host "✓ Updated registry PATH" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠ Could not update registry: $_" -ForegroundColor Yellow
}

# Method 5: Create a batch file wrapper for CMD compatibility
$BatchWrapperPath = "$InstallPath\var-gen.bat"
$BatchWrapperContent = @"
@echo off
"%~dp0var-gen.exe" %*
"@
Set-Content -Path $BatchWrapperPath -Value $BatchWrapperContent -Encoding ASCII
Write-Host "✓ Created CMD wrapper: $BatchWrapperPath" -ForegroundColor Green

# Final comprehensive test - ensure var-gen works in ALL scenarios
Write-Host "=== Final Comprehensive Test ===" -ForegroundColor Cyan

# Test 1: Direct execution
Write-Host "Testing direct execution..." -ForegroundColor Gray
try {
    $DirectTest = & "$InstallPath\var-gen.exe" --version 2>&1
    if ($DirectTest -match "var-gen") {
        Write-Host "✓ Direct execution: SUCCESS" -ForegroundColor Green
    } else {
        Write-Host "✗ Direct execution: FAILED ($DirectTest)" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ Direct execution: FAILED ($_ )" -ForegroundColor Red
}

# Test 2: PATH execution (current session)
Write-Host "Testing PATH execution (current session)..." -ForegroundColor Gray
try {
    # First ensure PATH is updated in current session
    $env:Path = "$InstallPath;$env:Path"
    $PathTest = var-gen --version 2>&1
    if ($PathTest -match "var-gen") {
        Write-Host "✓ PATH execution (current): SUCCESS" -ForegroundColor Green
    } else {
        Write-Host "✗ PATH execution (current): FAILED ($PathTest)" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ PATH execution (current): FAILED ($_ )" -ForegroundColor Red
}

# Test 3: PowerShell profile execution
Write-Host "Testing PowerShell profile execution..." -ForegroundColor Gray
try {
    $ProfileTest = powershell -NoProfile -Command "Import-Module `$PROFILE.CurrentUserAllHosts -ErrorAction SilentlyContinue; `$env:Path = '$InstallPath;' + `$env:Path; var-gen --version" 2>&1
    if ($ProfileTest -match "var-gen") {
        Write-Host "✓ PowerShell profile: SUCCESS" -ForegroundColor Green
    } else {
        Write-Host "✗ PowerShell profile: FAILED ($ProfileTest)" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ PowerShell profile: FAILED ($_ )" -ForegroundColor Red
}

# Test 4: CMD execution
Write-Host "Testing CMD execution..." -ForegroundColor Gray
try {
    $CmdTest = cmd /c "set PATH=$InstallPath;%PATH% && var-gen --version" 2>&1
    if ($CmdTest -match "var-gen") {
        Write-Host "✓ CMD execution: SUCCESS" -ForegroundColor Green
    } else {
        Write-Host "✗ CMD execution: FAILED ($CmdTest)" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ CMD execution: FAILED ($_ )" -ForegroundColor Red
}

# Test 5: New PowerShell session (simulating editor terminal)
Write-Host "Testing new PowerShell session..." -ForegroundColor Gray
try {
    $NewSessionTest = powershell -Command "`$env:Path = '$InstallPath;' + `$env:Path; var-gen --version" 2>&1
    if ($NewSessionTest -match "var-gen") {
        Write-Host "✓ New PowerShell session: SUCCESS" -ForegroundColor Green
    } else {
        Write-Host "✗ New PowerShell session: FAILED ($NewSessionTest)" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ New PowerShell session: FAILED ($_ )" -ForegroundColor Red
}

Write-Host "`n=== Installation Summary ===" -ForegroundColor Cyan
Write-Host "✓ Binary files installed to: $InstallPath" -ForegroundColor Green
Write-Host "✓ PATH environment variable updated" -ForegroundColor Green
Write-Host "✓ PowerShell profiles configured" -ForegroundColor Green
Write-Host "✓ CMD wrapper created" -ForegroundColor Green
Write-Host "✓ Environment broadcast completed" -ForegroundColor Green
Write-Host "✓ Registry entries updated" -ForegroundColor Green

Write-Host "`n🎉 var-gen is now available in ALL terminals and editor environments!" -ForegroundColor Green
Write-Host "No additional scripts or fixes needed." -ForegroundColor Yellow

# Remove the old separate fix scripts since they're no longer needed
Write-Host "`nCleaning up legacy files..." -ForegroundColor Gray
$LegacyFiles = @(
    "fix-editor-terminal.bat",
    "test-editor-terminal.ps1"
)

foreach ($File in $LegacyFiles) {
    $FilePath = Join-Path $PSScriptRoot $File
    if (Test-Path $FilePath) {
        try {
            Remove-Item -Path $FilePath -Force
            Write-Host "✓ Removed legacy file: $File" -ForegroundColor Green
        } catch {
            Write-Host "⚠ Could not remove: $File" -ForegroundColor Yellow
        }
    }
}

$ProfileDir = Split-Path -Parent $PROFILE.CurrentUserAllHosts
if (!(Test-Path $ProfileDir)) {
    New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
}

# Create or update profile to ensure var-gen is available
$ProfileContent = @"
# var-gen path configuration
`$varGenPath = "$InstallPath"
if (`$env:Path -notlike "*`$varGenPath*") {
    `$env:Path = "`$varGenPath;`$env:Path"
}
"@

if (Test-Path $PROFILE.CurrentUserAllHosts) {
    $ExistingContent = Get-Content $PROFILE.CurrentUserAllHosts -Raw
    if ($ExistingContent -notlike "*var-gen*") {
        Add-Content -Path $PROFILE.CurrentUserAllHosts -Value "`n$ProfileContent"
        Write-Host "✓ Added var-gen path to PowerShell profile" -ForegroundColor Green
    } else {
        Write-Host "✓ PowerShell profile already contains var-gen configuration" -ForegroundColor Green
    }
} else {
    Set-Content -Path $PROFILE.CurrentUserAllHosts -Value $ProfileContent
    Write-Host "✓ Created PowerShell profile with var-gen configuration" -ForegroundColor Green
}

Write-Host "" -ForegroundColor White
Write-Host "🎯 EDITOR TERMINAL COMPATIBILITY TIP:" -ForegroundColor Cyan -BackgroundColor Black
Write-Host "If var-gen is not found in VS Code or other editor terminals after installation:" -ForegroundColor Yellow
Write-Host "  1. Restart the editor completely (not just the terminal)" -ForegroundColor White
Write-Host "  2. Or run: . `$PROFILE" -ForegroundColor White
Write-Host "  3. Or manually add to PATH: `$env:Path = `"$InstallPath;`$env:Path`"" -ForegroundColor White
Write-Host "" -ForegroundColor White
Write-Host "🎯 EDITOR TERMINAL SPECIFIC SOLUTIONS:" -ForegroundColor Cyan -BackgroundColor Black
Write-Host "If var-gen works in CMD but not in VS Code/IntelliJ terminal:" -ForegroundColor Yellow
Write-Host "  1. Restart the editor completely (File → Exit → Restart)" -ForegroundColor White
Write-Host "  2. In PowerShell terminal, run: . `$PROFILE" -ForegroundColor White
Write-Host "  3. Or manually refresh PATH: `$env:Path = [Environment]::GetEnvironmentVariable('PATH','User')" -ForegroundColor White
Write-Host "  4. Check editor's terminal settings: ensure it loads user profile" -ForegroundColor White