# Go Backend

タスク管理APIのGoバックエンド実装です。Django REST Frameworkで実装された既存のバックエンドと同じ機能・APIを提供します。

## 技術スタック

- **言語**: Go 1.21+
- **フレームワーク**: Gin (高性能HTTPフレームワーク)
- **ORM**: GORM
- **データベース**: SQLite (開発) / PostgreSQL (本番)
- **認証**: トークンベース認証

## プロジェクト構造

```
go-backend/
├── cmd/
│   └── server/
│       └── main.go          # エントリーポイント
├── internal/
│   ├── config/
│   │   └── config.go        # 設定管理
│   ├── database/
│   │   └── database.go      # DB接続・マイグレーション
│   ├── handlers/
│   │   ├── auth.go          # 認証ハンドラー
│   │   └── tasks.go         # タスクハンドラー
│   ├── middleware/
│   │   ├── auth.go          # 認証ミドルウェア
│   │   └── cors.go          # CORSミドルウェア
│   └── models/
│       ├── user.go          # ユーザーモデル
│       ├── task.go          # タスクモデル
│       └── token.go         # トークンモデル
├── go.mod
├── go.sum
├── .env.example
├── start.sh                 # 起動スクリプト
├── dev.sh                   # 開発用スクリプト
└── README.md
```

## クイックスタート

### 前提条件

- Go 1.21以上がインストールされていること

### 起動方法

```bash
# 開発モード（推奨）
./dev.sh

# または、ビルドして起動
./start.sh
```

### 手動での起動

```bash
cd go-backend

# 依存関係のダウンロード
go mod download

# 開発サーバー起動
go run ./cmd/server

# または、ビルドして起動
go build -o bin/server ./cmd/server
./bin/server
```

## API エンドポイント

### 認証

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| POST | `/api/auth/register` | 不要 | ユーザー登録 |
| POST | `/api/auth/login` | 不要 | ログイン |
| POST | `/api/auth/logout` | 必要 | ログアウト |

### ユーザー

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/api/users/me` | 必要 | 現在のユーザー情報を取得 |

### タスク

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/api/tasks/` | 必要 | タスク一覧を取得 |
| POST | `/api/tasks/` | 必要 | タスクを作成 |
| GET | `/api/tasks/:id/` | 必要 | タスク詳細を取得 |
| PATCH | `/api/tasks/:id/` | 必要 | タスクを更新 |
| DELETE | `/api/tasks/:id/` | 必要 | タスクを削除 |

### ヘルスチェック

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/health` | 不要 | サーバーの状態確認 |

## 認証の使い方

### 1. ユーザー登録

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "securepassword123",
    "password2": "securepassword123"
  }'
```

レスポンス:
```json
{
  "token": "abc123...",
  "user": {
    "id": 1,
    "username": "testuser",
    "email": "test@example.com"
  }
}
```

### 2. ログイン

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "securepassword123"
  }'
```

### 3. 認証が必要なAPIへのアクセス

```bash
curl -X GET http://localhost:8080/api/tasks/ \
  -H "Authorization: Token abc123..."
```

## 環境変数

| 変数名 | デフォルト | 説明 |
|--------|-----------|------|
| `SECRET_KEY` | (開発用デフォルト) | トークン生成用シークレット |
| `DEBUG` | `true` | デバッグモード |
| `PORT` | `8080` | サーバーポート |
| `DATABASE_URL` | (空) | データベースURL（空の場合SQLite） |
| `CORS_ALLOWED_ORIGINS` | `http://localhost:3000` | 許可するオリジン |

## Djangoバックエンドとの互換性

このGoバックエンドは、既存のDjangoバックエンドと完全に互換性があります：

- 同じAPIエンドポイント構造
- 同じリクエスト/レスポンス形式
- 同じトークン認証方式 (`Authorization: Token <key>`)
- 同じエラーレスポンス形式

フロントエンドのAPI URLを変更するだけで、DjangoからGoに切り替えることができます。

## ベストプラクティス

### セキュリティ

- パスワードはbcryptでハッシュ化
- トークンは暗号学的に安全なランダム値
- 入力バリデーション実装済み
- SQLインジェクション対策（GORM使用）
- CORS設定可能

### パフォーマンス

- Ginフレームワークによる高速なHTTP処理
- GORMによる効率的なDB操作
- 適切なインデックス設定

## 本番環境へのデプロイ

```bash
# 環境変数を設定
export SECRET_KEY="your-production-secret-key"
export DEBUG="false"
export DATABASE_URL="postgresql://user:pass@host:5432/dbname"
export CORS_ALLOWED_ORIGINS="https://your-frontend.com"

# ビルド
go build -o bin/server ./cmd/server

# 起動
./bin/server
```

## ライセンス

MIT License
