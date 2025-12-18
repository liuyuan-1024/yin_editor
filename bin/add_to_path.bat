@echo off
chcp 65001 > nul 2>&1  # 解决中文乱码
setlocal enabledelayedexpansion

:: ==================== 核心配置 ====================
set "SCRIPT_DIR=%~dp0"  # 脚本所在目录（即ye.exe所在目录）
set "EXEC_NAME=ye.exe"  # 可执行文件名
set "TARGET_DIR=!SCRIPT_DIR!"  # 要添加到PATH的目录（即ye.exe所在目录）

:: 标准化路径（移除末尾反斜杠，避免PATH中出现双斜杠，不影响功能但更规范）
if "!TARGET_DIR:~-1!" equ "\" set "TARGET_DIR=!TARGET_DIR:~0,-1!"

:: ==================== 步骤1：检查可执行文件是否存在 ====================
if not exist "!TARGET_DIR!\!EXEC_NAME!" (
    echo 【错误】未找到可执行文件：!TARGET_DIR!\!EXEC_NAME!
    echo 请确认ye.exe已放在脚本所在目录下！
    pause
    exit /b 1
)

:: ==================== 步骤2：临时添加到当前会话PATH（立即生效） ====================
echo 【1/3】正在为当前会话临时添加路径...
:: 检查路径是否已存在，避免重复
echo ;!PATH!; | find /i ";!TARGET_DIR!;" > nul
if errorlevel 1 (
    set "PATH=!PATH!;!TARGET_DIR!"
    echo ✅ 临时添加成功！当前会话可直接使用ye命令。
) else (
    echo ℹ️  该路径已在当前会话PATH中，无需重复添加。
)

:: ==================== 步骤3：永久添加到用户级PATH（防截断+去重） ====================
echo.
echo 【2/3】正在为当前用户永久添加路径...
:: 从注册表读取**用户级**PATH（避免拼接系统PATH导致截断）
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v PATH 2^>nul') do set "USER_PATH=%%b"
if not defined USER_PATH set "USER_PATH="  # 若用户级PATH为空，初始化为空

:: 检查用户级PATH中是否已有目标路径
echo ;!USER_PATH!; | find /i ";!TARGET_DIR!;" > nul
if errorlevel 1 (
    :: 仅拼接用户级PATH，避免字符过长
    if defined USER_PATH (
        setx PATH "!USER_PATH!;!TARGET_DIR!" > nul 2>&1
    ) else (
        setx PATH "!TARGET_DIR!" > nul 2>&1
    )

    if !errorlevel! equ 0 (
        echo ✅ 永久添加成功！重启终端后全局生效。
    ) else (
        echo ❌ 永久添加失败！可能是PATH字符长度超限，请手动添加：!TARGET_DIR!
    )
) else (
    echo ℹ️  该路径已在用户级PATH中，无需重复添加。
)

:: ==================== 步骤4：提示PowerShell生效方法 ====================
echo.
echo 【3/3】PowerShell生效提示：
echo - 当前CMD会话已可直接用ye命令；
echo - PowerShell需执行：$env:PATH += ";!TARGET_DIR!"（临时），或重启PowerShell（永久）；
echo - 也可重启电脑后，所有终端均生效。

endlocal
pause
