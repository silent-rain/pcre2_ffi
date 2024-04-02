#!/bin/bash

# 检查 nc 命令是否存在
if ! command -v nc &> /dev/null
then
    echo "nc (netcat) 未安装，正在尝试安装..."

    # 尝试使用 pacman 安装 nc
    if command -v pacman &> /dev/null
    then
        sudo pacman update
        sudo pacman install netcat
    # 尝试使用 apt-get
    elif command -v apt-get &> /dev/null
    then
        sudo apt-get install netcat
    # 尝试使用 yum
    elif command -v yum &> /dev/null
    then
        sudo yum install nc
    else
        echo "未找到合适的包管理器。请手动安装 nc。"
        exit 1
    fi
fi


# 定义接收 UDP 数据的端口
PORT=34254

echo "开始监听端口: $PORT"
# 使用 nc (netcat) 监听指定端口的 UDP 数据
nc -lu 127.0.0.1 $PORT
# nc -luv 127.0.0.1 $PORT
# nc -nluv 127.0.0.1 $PORT

# 使用 nc (netcat) 监听指定端口的 UDP 数据，并打印接收到的每条消息
# while IFS= read -r line; do
#     echo "接收到: $line"
# done < <(nc -nluv 127.0.0.1 $PORT)