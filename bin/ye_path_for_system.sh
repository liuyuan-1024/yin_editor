#!/bin/bash
set -euo pipefail

# ==================== 核心配置（无需修改，自动识别） ====================
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
EXEC_DIR="${SCRIPT_DIR}"
EXEC_NAME="ye"
# 系统级脚本路径（/etc/profile.d/下的专属sh文件，推荐命名为ye_path.sh）
SYS_PROFILE_FILE="/etc/profile.d/ye_path.sh"

# ==================== 辅助函数：颜色输出 ====================
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

if [ ! -x "${EXEC_PATH}" ]; then
    warn "可执行文件${EXEC_NAME}缺少执行权限，正在尝试添加..."
    chmod +x "${EXEC_PATH}" || error "添加执行权限失败，请手动执行：chmod +x ${EXEC_PATH}"
    info "已成功为${EXEC_NAME}添加执行权限！"
fi

# ==================== 步骤2：临时添加到当前会话PATH（立即生效） ====================
info "正在为当前会话临时添加路径：${EXEC_DIR}"
if [[ ":$PATH:" != *":${EXEC_DIR}:"* ]]; then
    export PATH="$PATH:${EXEC_DIR}"
    info "✅ 临时添加成功！当前会话可直接使用${EXEC_NAME}命令。"
else
    info "ℹ️  该路径已在当前会话PATH中，无需重复添加。"
fi

# ==================== 步骤3：系统级添加（/etc/profile.d/*.sh） ====================
info "\n正在为所有用户添加系统级PATH到：${SYS_PROFILE_FILE}"

# 检查是否有root权限（系统级修改必须）
if [ "$(id -u)" -ne 0 ]; then
    error "添加系统级PATH需要root权限，请用sudo运行脚本！\n示例：sudo ./add_to_path.sh"
fi

# 定义要写入系统级脚本的内容
ADD_LINE="export PATH=\"\$PATH:${EXEC_DIR}\""
COMMENT_LINE="# Add ${EXEC_NAME} directory to system PATH for all users"

# 检查系统级脚本中是否已有相同内容
if [ -f "${SYS_PROFILE_FILE}" ]; then
    if grep -qxF "${ADD_LINE}" "${SYS_PROFILE_FILE}" 2>/dev/null; then
        info "ℹ️  该路径已在${SYS_PROFILE_FILE}中，无需重复添加。"
    else
        # 追加内容到现有脚本
        echo -e "\n${COMMENT_LINE}" >> "${SYS_PROFILE_FILE}"
        echo "${ADD_LINE}" >> "${SYS_PROFILE_FILE}"
        info "✅ 系统级PATH添加成功！"
    fi
else
    # 新建系统级脚本并写入内容
    echo "${COMMENT_LINE}" > "${SYS_PROFILE_FILE}"
    echo "${ADD_LINE}" >> "${SYS_PROFILE_FILE}"
    chmod 644 "${SYS_PROFILE_FILE}"  # 设置合理权限（所有用户可读，root可写）
    info "✅ 系统级脚本${SYS_PROFILE_FILE}已创建并添加PATH！"
fi

# ==================== 最终提示 ====================
info "\n🎉 操作完成！验证方法："
echo "  1. 当前会话：输入 ${EXEC_NAME} --help（或直接运行${EXEC_NAME}）"
echo "  2. 新会话（所有用户）：重启终端后输入 ${EXEC_NAME}"
echo "  3. 系统级生效：若需立即生效，执行：source ${SYS_PROFILE_FILE}"
