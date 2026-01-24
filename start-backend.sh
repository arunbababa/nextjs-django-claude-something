#!/bin/bash

# バックエンド起動スクリプト
echo "================================"
echo "バックエンド (Django) を起動します"
echo "================================"

cd "$(dirname "$0")/backend"

# Python のバージョンチェック
if ! command -v python3 &> /dev/null; then
    echo "エラー: Python 3 がインストールされていません"
    exit 1
fi

echo "Python バージョン: $(python3 --version)"

# 依存関係のインストール確認
echo ""
echo "依存関係を確認中..."
pip3 install -r requirements.txt --quiet

# データベースマイグレーション
echo ""
echo "データベースマイグレーションを実行中..."
python3 manage.py migrate

# 開発サーバー起動
echo ""
echo "================================"
echo "Django開発サーバーを起動します"
echo "URL: http://localhost:8000"
echo "停止: Ctrl+C"
echo "================================"
python3 manage.py runserver
