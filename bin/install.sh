#!/bin/bash
# 将编译好的 ye 复制到 /usr/local/bin（通常已在 PATH 中）
sudo cp target/release/ye /usr/local/bin/
if [ $? -eq 0 ]; then
    echo "安装成功！可直接在终端输入 `ye` 启动编辑器"
else
    echo "安装失败，请检查权限"
fi
