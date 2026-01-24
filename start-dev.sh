#!/bin/bash

# 開発環境一括起動スクリプト
echo "========================================"
echo "開発環境を起動します"
echo "========================================"

# スクリプトのディレクトリに移動
cd "$(dirname "$0")"

# バックエンドとフロントエンドを並行起動
echo ""
echo "バックエンドとフロントエンドを並行起動します..."
echo "停止: Ctrl+C"
echo ""

# バックエンドをバックグラウンドで起動
./start-backend.sh &
BACKEND_PID=$!

# 少し待機
sleep 2

# フロントエンドを起動（フォアグラウンド）
./start-frontend.sh &
FRONTEND_PID=$!

# トラップ: Ctrl+Cで両方のプロセスを終了
trap "echo ''; echo 'サーバーを停止しています...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit" INT TERM

# 両方のプロセスが終了するまで待機
wait
