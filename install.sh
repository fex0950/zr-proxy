#!/bin/sh
# zr-proxy 一键安装脚本
# 用法: curl -fsSL https://raw.githubusercontent.com/fex0950/zr-proxy/master/install.sh | sh
set -e

REPO="fex0950/zr-proxy"
BINARY="zr-proxy"

main() {
    os="$(uname -s)"
    arch="$(uname -m)"

    if [ "$os" != "Darwin" ]; then
        echo "错误: zr-proxy 目前仅支持 macOS（当前系统: $os）" >&2
        exit 1
    fi

    case "$arch" in
        arm64)  target="aarch64-apple-darwin" ;;
        x86_64) target="x86_64-apple-darwin" ;;
        *)
            echo "错误: 不支持的架构 $arch" >&2
            exit 1
            ;;
    esac

    echo "正在获取最新版本..."
    version="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')"
    if [ -z "$version" ]; then
        echo "错误: 无法获取最新版本信息" >&2
        exit 1
    fi

    url="https://github.com/$REPO/releases/download/$version/$BINARY-$version-$target.tar.gz"
    echo "正在下载 $BINARY $version ($target)..."

    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT

    curl -fsSL "$url" -o "$tmp/$BINARY.tar.gz"
    tar -xzf "$tmp/$BINARY.tar.gz" -C "$tmp"

    if [ -w /usr/local/bin ]; then
        dest="/usr/local/bin"
    else
        dest="$HOME/.local/bin"
        mkdir -p "$dest"
    fi

    mv "$tmp/$BINARY" "$dest/$BINARY"
    chmod +x "$dest/$BINARY"

    echo "安装完成: $dest/$BINARY"

    case ":$PATH:" in
        *":$dest:"*) ;;
        *)
            echo "提示: $dest 不在 PATH 中，请将以下内容加入 ~/.zshrc 后重开终端:"
            echo "  export PATH=\"$dest:\$PATH\""
            ;;
    esac

    echo "现在可以在任意目录运行: zr-proxy"
}

main
