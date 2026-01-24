#!/bin/bash

# フロントエンド起動スクリプト
echo "================================"
echo "フロントエンド (Next.js) を起動します"
echo "================================"

cd "$(dirname "$0")/frontend"

# Node.js のバージョンチェック
if ! command -v node &> /dev/null; then
    echo "エラー: Node.js がインストールされていません"
    exit 1
fi

echo "Node.js バージョン: $(node --version)"
echo "npm バージョン: $(npm --version)"

# 依存関係のインストール確認
if [ ! -d "node_modules" ]; then
    echo ""
    echo "依存関係をインストール中..."
    npm install
else
    echo ""
    echo "依存関係は既にインストール済みです"
fi

# 開発サーバー起動
echo ""
echo "================================"
echo "Next.js開発サーバーを起動します"
echo "URL: http://localhost:3000"
echo "停止: Ctrl+C"
echo "================================"
npm run dev
