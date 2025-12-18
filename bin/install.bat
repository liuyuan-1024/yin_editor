@echo off
:: 将编译好的 ye.exe 复制到系统目录（需管理员权限）
copy target\release\ye.exe C:\Windows\System32\
if %errorlevel% equ 0 (
    echo 安装成功！可直接在命令行输入 `ye` 启动编辑器
) else (
    echo 安装失败，请以管理员身份运行此脚本
)
