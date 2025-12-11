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
    Remove-Item -Path $InstallPath -Recurse -Force
    Write-Host "已删除安装目录: $InstallPath" -ForegroundColor Green
} catch {
    Write-Host "删除安装目录时出错: $_" -ForegroundColor Red
}

# 删除开始菜单快捷方式
$ShortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\var-gen.lnk"
if (Test-Path $ShortcutPath) {
    try {
        Remove-Item -Path $ShortcutPath -Force
        Write-Host "已删除开始菜单快捷方式" -ForegroundColor Green
    } catch {
        Write-Host "删除快捷方式时出错: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "开始菜单快捷方式不存在" -ForegroundColor Gray
}

Write-Host "=== 卸载完成！ ===" -ForegroundColor Green
Write-Host "请重新打开命令提示符或PowerShell以使环境变量更改生效" -ForegroundColor Yellow

# 验证卸载
Write-Host "验证卸载结果..." -ForegroundColor Green
if (Test-Path $InstallPath) {
    Write-Host "警告: 安装目录仍然存在" -ForegroundColor Yellow
} else {
    Write-Host "✓ 安装目录已成功删除" -ForegroundColor Green
}

# 检查PATH是否已清理
$UpdatedPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UpdatedPath -like "*var-gen*") {
    Write-Host "警告: PATH中可能仍包含var-gen相关路径" -ForegroundColor Yellow
} else {
    Write-Host "✓ 用户PATH已清理" -ForegroundColor Green
}