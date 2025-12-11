# var-gen 用户目录卸载脚本 (Windows PowerShell)
# 无需管理员权限，卸载用户安装的var-gen

Write-Host "=== var-gen 用户目录卸载程序 ===" -ForegroundColor Yellow

$InstallPath = "$env:LOCALAPPDATA\var-gen"

# 检查是否已安装
if (-not (Test-Path $InstallPath)) {
    Write-Host "var-gen 未在用户目录安装或安装路径不存在" -ForegroundColor Red
    Write-Host "安装路径: $InstallPath" -ForegroundColor Gray
    exit 1
}

Write-Host "正在卸载 var-gen..." -ForegroundColor Yellow

# 从用户PATH中移除
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -like "*$InstallPath*") {
    $NewPath = $CurrentPath -replace [regex]::Escape(";$InstallPath"), ""
    $NewPath = $NewPath -replace [regex]::Escape("$InstallPath;"), ""
    $NewPath = $NewPath -replace [regex]::Escape("$InstallPath"), ""
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "已从用户PATH中移除" -ForegroundColor Green
} else {
    Write-Host "安装路径不在用户PATH中" -ForegroundColor Yellow
}

# 删除安装目录
try {
    # 尝试使用cmd的rd命令删除，有时比PowerShell的Remove-Item更可靠
    if (Test-Path $InstallPath) {
        & cmd /c "rd /s /q `"$InstallPath`"" 2>$null
        if (Test-Path $InstallPath) {
            # 如果cmd删除失败，再尝试PowerShell方式
            Remove-Item -Path $InstallPath -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    
    if (Test-Path $InstallPath) {
        Write-Host "警告: 无法完全删除安装目录，可能需要手动删除: $InstallPath" -ForegroundColor Yellow
    } else {
        Write-Host "已删除安装目录: $InstallPath" -ForegroundColor Green
    }
} catch {
    Write-Host "删除安装目录时出错: $_" -ForegroundColor Red
}

# 删除开始菜单快捷方式
$ShortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\var-gen.lnk"
if (Test-Path $ShortcutPath) {
    try {
        Remove-Item -Path $ShortcutPath -Force -ErrorAction SilentlyContinue
        Write-Host "已删除开始菜单快捷方式" -ForegroundColor Green
    } catch {
        Write-Host "删除快捷方式时出错: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "开始菜单快捷方式不存在" -ForegroundColor Gray
}

Write-Host "=== 卸载完成！ ===" -ForegroundColor Green
Write-Host "请重新打开命令提示符或PowerShell以使环境变量更改生效" -ForegroundColor Yellow

# 最终建议
Write-Host "" 
Write-Host "卸载完成后的建议:" -ForegroundColor Cyan
Write-Host "1. 重新打开命令提示符或PowerShell" -ForegroundColor Gray
Write-Host "2. 运行 'var-gen --version' 确认卸载成功" -ForegroundColor Gray
if (Test-Path $InstallPath) {
    Write-Host "3. 如需要，请手动删除剩余文件: $InstallPath" -ForegroundColor Gray
}

# 验证卸载
Write-Host "验证卸载结果..." -ForegroundColor Green
if (Test-Path $InstallPath) {
    Write-Host "警告: 安装目录仍然存在" -ForegroundColor Yellow
} else {
    Write-Host " 安装目录已成功删除" -ForegroundColor Green
}

# 检查PATH是否已清理
$UpdatedPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UpdatedPath -like "*var-gen*") {
    Write-Host "警告: PATH中可能仍包含var-gen相关路径" -ForegroundColor Yellow
} else {
    Write-Host " 用户PATH已清理" -ForegroundColor Green
}
