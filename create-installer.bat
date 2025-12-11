@echo off
chcp 65001 >nul
echo === 构建 var-gen 安装包 ===

:: 设置变量
set VERSION=0.2.0
set BUILDDIR=windows-installer
set PACKAGEDIR=%BUILDDIR%\var-gen-%VERSION%

:: 清理旧的构建目录
if exist "%BUILDDIR%" rd /s /q "%BUILDDIR%"

:: 创建构建目录结构
mkdir "%PACKAGEDIR%"
mkdir "%PACKAGEDIR%\bin"
mkdir "%PACKAGEDIR%\scripts"

echo 正在编译项目...
cargo build --release

if %errorlevel% neq 0 (
    echo 编译失败！
    pause
    exit /b 1
)

echo 正在复制文件...

:: 复制二进制文件
copy "target\release\var-gen.exe" "%PACKAGEDIR%\bin\" >nul

:: 复制安装脚本（使用PowerShell保持UTF-8编码）
powershell -Command "Get-Content 'install.ps1' -Encoding UTF8 | Set-Content '%PACKAGEDIR%\scripts\install.ps1' -Encoding UTF8"
powershell -Command "Get-Content 'uninstall.ps1' -Encoding UTF8 | Set-Content '%PACKAGEDIR%\scripts\uninstall.ps1' -Encoding UTF8"

:: 复制文档文件
copy "README.md" "%PACKAGEDIR%\" >nul
copy "mapping-config-example.json" "%PACKAGEDIR%\" >nul

:: 创建安装说明
echo # var-gen v%VERSION% Installation Package > "%PACKAGEDIR%\INSTALL.md"
echo. >> "%PACKAGEDIR%\INSTALL.md"
echo ## Quick Installation >> "%PACKAGEDIR%\INSTALL.md"
echo 1. Extract this zip file >> "%PACKAGEDIR%\INSTALL.md"
echo 2. Run PowerShell as Administrator >> "%PACKAGEDIR%\INSTALL.md"
echo 3. Execute: .\scripts\install.ps1 >> "%PACKAGEDIR%\INSTALL.md"
echo. >> "%PACKAGEDIR%\INSTALL.md"
echo ## File Description >> "%PACKAGEDIR%\INSTALL.md"
echo - bin\var-gen.exe - Main program >> "%PACKAGEDIR%\INSTALL.md"
echo - scripts\install.ps1 - Installation script >> "%PACKAGEDIR%\INSTALL.md"
echo - scripts\uninstall.ps1 - Uninstallation script >> "%PACKAGEDIR%\INSTALL.md"
echo - uninstall.bat - One-click uninstall (double-click to run) >> "%PACKAGEDIR%\INSTALL.md"
echo - mapping-config-example.json - Configuration example >> "%PACKAGEDIR%\INSTALL.md"
echo. >> "%PACKAGEDIR%\INSTALL.md"
echo After installation, run in any directory: var-gen --help >> "%PACKAGEDIR%\INSTALL.md"

:: 创建一键安装批处理
echo @echo off > "%PACKAGEDIR%\install.bat"
echo :: var-gen 一键安装脚本 >> "%PACKAGEDIR%\install.bat"
echo :: 直接运行此脚本即可安装 var-gen >> "%PACKAGEDIR%\install.bat"
echo powershell -ExecutionPolicy Bypass -File "%%~dp0scripts\install.ps1" >> "%PACKAGEDIR%\install.bat"
echo pause >> "%PACKAGEDIR%\install.bat"

:: 创建一键卸载批处理
echo @echo off > "%PACKAGEDIR%\uninstall.bat"
echo chcp 65001 ^>nul 2^>^&1 >> "%PACKAGEDIR%\uninstall.bat"
echo title var-gen Uninstaller >> "%PACKAGEDIR%\uninstall.bat"
echo color 0A >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo echo. >> "%PACKAGEDIR%\uninstall.bat"
echo echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo echo    var-gen Uninstaller >> "%PACKAGEDIR%\uninstall.bat"
echo echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo echo. >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo :: Check if PowerShell is available >> "%PACKAGEDIR%\uninstall.bat"
echo where powershell ^>nul 2^>^&1 >> "%PACKAGEDIR%\uninstall.bat"
echo if %%errorlevel%% neq 0 ( >> "%PACKAGEDIR%\uninstall.bat"
echo     echo Error: PowerShell not found, cannot perform uninstall >> "%PACKAGEDIR%\uninstall.bat"
echo     pause >> "%PACKAGEDIR%\uninstall.bat"
echo     exit /b 1 >> "%PACKAGEDIR%\uninstall.bat"
echo ) >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo echo Preparing to uninstall var-gen... >> "%PACKAGEDIR%\uninstall.bat"
echo echo. >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo :: Execute PowerShell uninstall script >> "%PACKAGEDIR%\uninstall.bat"
echo echo Executing uninstall operation... >> "%PACKAGEDIR%\uninstall.bat"
echo cd /d "%%~dp0" >> "%PACKAGEDIR%\uninstall.bat"
echo powershell -ExecutionPolicy Bypass -Command "& {.\scripts\uninstall.ps1}" >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo if %%errorlevel%% equ 0 ( >> "%PACKAGEDIR%\uninstall.bat"
echo     echo. >> "%PACKAGEDIR%\uninstall.bat"
echo     echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo     echo    Uninstall Complete! >> "%PACKAGEDIR%\uninstall.bat"
echo     echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo     echo. >> "%PACKAGEDIR%\uninstall.bat"
echo     echo Uninstall process completed. >> "%PACKAGEDIR%\uninstall.bat"
echo     echo Please reopen Command Prompt or PowerShell to make environment variable changes effective. >> "%PACKAGEDIR%\uninstall.bat"
echo ) else ( >> "%PACKAGEDIR%\uninstall.bat"
echo     echo. >> "%PACKAGEDIR%\uninstall.bat"
echo     echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo     echo    Error during uninstall process >> "%PACKAGEDIR%\uninstall.bat"
echo     echo ========================================== >> "%PACKAGEDIR%\uninstall.bat"
echo     echo. >> "%PACKAGEDIR%\uninstall.bat"
echo     echo Please check error messages and handle manually. >> "%PACKAGEDIR%\uninstall.bat"
echo ) >> "%PACKAGEDIR%\uninstall.bat"
echo. >> "%PACKAGEDIR%\uninstall.bat"
echo echo. >> "%PACKAGEDIR%\uninstall.bat"
echo echo Press any key to exit... >> "%PACKAGEDIR%\uninstall.bat"
echo pause ^>nul >> "%PACKAGEDIR%\uninstall.bat"

echo 正在创建安装包...
powershell -Command "Compress-Archive -Path '%PACKAGEDIR%\*' -DestinationPath '%BUILDDIR%\var-gen-%VERSION%-installer.zip' -Force"

echo === 构建完成！ ===
echo 安装包位置: %BUILDDIR%\var-gen-%VERSION%-installer.zip
echo 解压后运行 install.bat 即可开始安装
pause