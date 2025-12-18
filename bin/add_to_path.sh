#!/bin/bash
set -euo pipefail

# ==================== 核心配置（无需修改，自动识别） ====================
# 获取脚本所在目录的绝对路径（解决符号链接、相对路径问题）
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
EXEC_DIR="${SCRIPT_DIR}"  # 可执行文件与脚本同目录
EXEC_NAME="ye"            # 可执行文件名（Linux/macOS无.exe后缀，若有则改为ye.exe）

# ==================== 辅助函数：颜色输出（增强可读性） ====================
info() {
    echo -e "\033[32m[INFO]\033[0m $1"
}
warn() {
    echo -e "\033[33m[WARN]\033[0m $1"
}
error() {
    echo -e "\033[31m[ERROR]\033[0m $1"
    exit 1
}

# ==================== 步骤1：检查可执行文件 ====================
EXEC_PATH="${EXEC_DIR}/${EXEC_NAME}"
if [ ! -f "${EXEC_PATH}" ]; then
    error "未找到可执行文件：${EXEC_PATH}\n请确认文件已放在脚本所在目录下！"
fi

# 检查可执行权限，若无则尝试添加
if [ ! -x "${EXEC_PATH}" ]; then
    warn "可执行文件${EXEC_NAME}缺少执行权限，正在尝试添加..."
    chmod +x "${EXEC_PATH}" || error "添加执行权限失败，请手动执行：chmod +x ${EXEC_PATH}"
    info "已成功为${EXEC_NAME}添加执行权限！"
fi

# ==================== 步骤2：临时添加到当前会话PATH（立即生效） ====================
info "正在为当前会话临时添加路径：${EXEC_DIR}"
# 准确检查路径是否已存在（用:$PATH:包裹，避免子串匹配错误）
if [[ ":$PATH:" != *":${EXEC_DIR}:"* ]]; then
    export PATH="$PATH:${EXEC_DIR}"
    info "✅ 临时添加成功！当前会话可直接使用${EXEC_NAME}命令。"
else
    info "ℹ️  该路径已在当前会话PATH中，无需重复添加。"
fi

# ==================== 步骤3：确定Shell配置文件（兼容Bash/Zsh/macOS/Linux） ====================
# 优先级：Zsh(.zshrc) > Bash(.bash_profile/macOS) > Bash(.bashrc/Linux) > .profile
if [ -n "${ZSH_VERSION:-}" ]; then
    CONFIG_FILE="$HOME/.zshrc"
elif [ -n "${BASH_VERSION:-}" ]; then
    # macOS的Bash默认读取.bash_profile，Linux默认读取.bashrc
    if [ "$(uname -s)" = "Darwin" ]; then
        CONFIG_FILE="$HOME/.bash_profile"
    else
        CONFIG_FILE="$HOME/.bashrc"
    fi
    # 若首选配置文件不存在，降级到.profile
    if [ ! -f "${CONFIG_FILE}" ]; then
        CONFIG_FILE="$HOME/.profile"
    fi
else
    CONFIG_FILE="$HOME/.profile"
fi

# ==================== 步骤4：永久添加到配置文件（去重+防重复写入） ====================
info "\n正在为当前用户永久添加路径到：${CONFIG_FILE}"
# 检查配置文件中是否已有相同的PATH添加语句
ADD_LINE="export PATH=\"\$PATH:${EXEC_DIR}\""
if grep -qxF "${ADD_LINE}" "${CONFIG_FILE}" 2>/dev/null; then
    info "ℹ️  该路径已在${CONFIG_FILE}中，无需重复添加。"
else
    # 若配置文件不存在则创建，存在则追加
    echo -e "\n# Add ${EXEC_NAME} directory to PATH" >> "${CONFIG_FILE}"
    echo "${ADD_LINE}" >> "${CONFIG_FILE}"
    info "✅ 永久添加成功！"
    info "请运行以下命令立即生效，或重启终端："
    echo "    source ${CONFIG_FILE}"
fi

# ==================== 最终提示 ====================
info "\n🎉 操作完成！验证方法："
echo "  1. 当前会话：输入 ${EXEC_NAME} --help（或直接运行${EXEC_NAME}）"
echo "  2. 新会话：重启终端后输入 ${EXEC_NAME}"
