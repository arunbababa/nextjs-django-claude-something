# Rust Backend

タスク管理APIと**お金の貸し借り管理API**のRustバックエンド実装です。Django/Go/Jakarta EEで実装された既存のバックエンドと同じタスク管理機能を提供し、さらにお金の貸し借り管理機能を追加しています。

## 技術スタック

- **言語**: Rust 1.70+
- **フレームワーク**: Actix-web 4 (高性能HTTPフレームワーク)
- **データベース**: SQLx (SQLite / PostgreSQL対応)
- **認証**: トークンベース認証
- **パスワードハッシュ**: Argon2
- **バリデーション**: validator

## 機能

### 1. タスク管理 (Todo)
- タスクの作成・取得・更新・削除
- 完了状態の管理
- ユーザーごとのタスク分離

### 2. お金の貸し借り管理 (Debt) ★新機能
- 貸し借り記録の作成・取得・更新・削除
- 「貸した」「借りた」の区別
- 精算（settle）機能
- サマリー（貸し借りの総額、差し引き）

## プロジェクト構造

```
rust-backend/
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── config/
│   │   └── mod.rs           # 設定管理
│   ├── db/
│   │   └── mod.rs           # DB接続・マイグレーション
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs          # 認証ハンドラー
│   │   ├── users.rs         # ユーザーハンドラー
│   │   ├── tasks.rs         # タスクハンドラー
│   │   └── debts.rs         # 貸し借りハンドラー
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs          # 認証ミドルウェア
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs          # ユーザーモデル
│   │   ├── task.rs          # タスクモデル
│   │   ├── token.rs         # トークンモデル
│   │   └── debt.rs          # 貸し借りモデル
│   └── routes/
│       └── mod.rs           # ルーティング
├── Cargo.toml
├── .env.example
├── start.sh                 # 起動スクリプト
├── dev.sh                   # 開発用スクリプト
└── README.md
```

## クイックスタート

### 前提条件

- **Rust 1.70** 以上

### インストール

```bash
# Rustをインストール（未インストールの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 起動方法

```bash
cd rust-backend

# 開発モード（推奨）
./dev.sh

# または、リリースビルドして起動
./start.sh

# 手動で起動
cargo run
```

サーバーは `http://localhost:8082` で起動します。

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

### タスク管理

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/api/tasks/` | 必要 | タスク一覧を取得 |
| POST | `/api/tasks/` | 必要 | タスクを作成 |
| GET | `/api/tasks/:id/` | 必要 | タスク詳細を取得 |
| PATCH | `/api/tasks/:id/` | 必要 | タスクを更新 |
| DELETE | `/api/tasks/:id/` | 必要 | タスクを削除 |

### お金の貸し借り管理 ★新機能

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/api/debts/` | 必要 | 貸し借り一覧を取得 |
| POST | `/api/debts/` | 必要 | 貸し借りを記録 |
| GET | `/api/debts/:id/` | 必要 | 貸し借り詳細を取得 |
| PATCH | `/api/debts/:id/` | 必要 | 貸し借りを更新 |
| DELETE | `/api/debts/:id/` | 必要 | 貸し借りを削除 |
| POST | `/api/debts/:id/settle/` | 必要 | 精算済みにする |
| GET | `/api/debts/summary/` | 必要 | サマリーを取得 |

### ヘルスチェック

| メソッド | エンドポイント | 認証 | 説明 |
|---------|---------------|------|------|
| GET | `/api/health` | 不要 | サーバーの状態確認 |

## 貸し借り管理の使い方

### 1. 貸し借りを記録

```bash
# お金を貸した場合
curl -X POST http://localhost:8082/api/debts/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Token abc123..." \
  -d '{
    "counterparty": "田中さん",
    "amount": 5000,
    "debt_type": "lent",
    "description": "ランチ代立て替え"
  }'

# お金を借りた場合
curl -X POST http://localhost:8082/api/debts/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Token abc123..." \
  -d '{
    "counterparty": "佐藤さん",
    "amount": 3000,
    "debt_type": "borrowed",
    "description": "映画チケット代"
  }'
```

レスポンス:
```json
{
  "id": 1,
  "counterparty": "田中さん",
  "amount": 5000,
  "debt_type": "lent",
  "description": "ランチ代立て替え",
  "is_settled": false,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z",
  "settled_at": null,
  "user": 1
}
```

### 2. 精算する

```bash
curl -X POST http://localhost:8082/api/debts/1/settle/ \
  -H "Authorization: Token abc123..."
```

### 3. サマリーを取得

```bash
curl http://localhost:8082/api/debts/summary/ \
  -H "Authorization: Token abc123..."
```

レスポンス:
```json
{
  "total_lent": 15000,
  "total_borrowed": 8000,
  "net_balance": 7000,
  "unsettled_lent": 5000,
  "unsettled_borrowed": 3000
}
```

- `total_lent`: 貸している総額
- `total_borrowed`: 借りている総額
- `net_balance`: 差し引き（正なら貸し越し、負なら借り越し）
- `unsettled_lent`: 未精算の貸し
- `unsettled_borrowed`: 未精算の借り

## 環境変数

| 変数名 | デフォルト | 説明 |
|--------|-----------|------|
| `DATABASE_URL` | `sqlite:data.db?mode=rwc` | データベースURL |
| `HOST` | `127.0.0.1` | サーバーホスト |
| `PORT` | `8082` | サーバーポート |
| `DEBUG` | `true` | デバッグモード |
| `CORS_ALLOWED_ORIGINS` | `http://localhost:3000` | 許可するオリジン |

## 他のバックエンドとの互換性

タスク管理APIは、既存のDjango/Go/Jakarta EEバックエンドと完全に互換性があります：

- 同じAPIエンドポイント構造
- 同じリクエスト/レスポンス形式
- 同じトークン認証方式 (`Authorization: Token <key>`)

**お金の貸し借り管理APIはRustバックエンドのみの新機能です。**

## ベストプラクティス

### セキュリティ
- パスワードはArgon2でハッシュ化
- トークンは暗号学的に安全なランダム値
- SQLインジェクション対策（SQLx使用）
- 入力バリデーション

### パフォーマンス
- Actix-webによる高性能なHTTP処理
- SQLxによる非同期DB操作
- 接続プール設定

## 本番環境へのデプロイ

```bash
# 環境変数を設定
export DATABASE_URL="postgresql://user:pass@host:5432/dbname"
export DEBUG="false"
export CORS_ALLOWED_ORIGINS="https://your-frontend.com"

# リリースビルド
cargo build --release

# 起動
./target/release/rust-backend
```

## ライセンス

MIT License
