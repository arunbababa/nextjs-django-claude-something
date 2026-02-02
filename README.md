# タスク管理アプリケーション

Next.js (フロントエンド) と Django/Go/Jakarta EE/Rust (バックエンド) で作成したタスク管理アプリケーションです。

**バックエンドは Django、Go、Jakarta EE、Rust の4つの実装があります。** どれも同じタスク管理APIを提供しており、好みに応じて選択できます。

**★ Rustバックエンドにはお金の貸し借り管理機能も追加されています。**

## 機能

- ユーザー新規登録
- ユーザーログイン
- タスクの追加
- タスクの編集
- タスクの削除
- タスクの完了状態の切り替え

## 技術スタック

### バックエンド (Django)
- Django 5.0.1
- Django REST Framework
- Token認証
- SQLite データベース
- CORS設定

### バックエンド (Go)
- Go 1.21+
- Gin (高性能HTTPフレームワーク)
- GORM (ORM)
- Token認証
- SQLite / PostgreSQL対応
- CORS設定

### バックエンド (Jakarta EE)
- Java 17+
- Jakarta EE 10
- JAX-RS (REST API)
- JPA + Hibernate
- CDI (依存性注入)
- Payara Micro
- H2 / PostgreSQL対応

### バックエンド (Rust) - 新規追加
- Rust 1.70+
- Actix-web 4 (高性能HTTPフレームワーク)
- SQLx (非同期DB)
- Token認証 (Argon2)
- SQLite / PostgreSQL対応
- ★ お金の貸し借り管理機能

### フロントエンド
- Next.js 16 (App Router)
- TypeScript
- Tailwind CSS
- Shadcn UI コンポーネント

## API仕様

### 認証関連
- `POST /api/auth/register` - ユーザー新規登録
- `POST /api/auth/login` - ログイン（トークン取得）
- `POST /api/auth/logout` - ログアウト

### ユーザー関連
- `GET /api/users/me` - 現在のユーザー情報取得

### タスク関連
- `GET /api/tasks/` - タスク一覧取得
- `POST /api/tasks/` - タスク作成
- `GET /api/tasks/{id}/` - タスク詳細取得
- `PATCH /api/tasks/{id}/` - タスク部分更新
- `DELETE /api/tasks/{id}/` - タスク削除

## セットアップ

### バックエンド（Django）

1. バックエンドディレクトリに移動:
```bash
cd backend
```

2. 依存パッケージをインストール:
```bash
pip install -r requirements.txt
```

3. マイグレーションを実行:
```bash
python manage.py migrate
```

4. 開発サーバーを起動:
```bash
python manage.py runserver
```

バックエンドは `http://localhost:8000` で起動します。

### バックエンド（Go）

1. Goバックエンドディレクトリに移動:
```bash
cd go-backend
```

2. 開発サーバーを起動（簡単な方法）:
```bash
./dev.sh
```

または、手動で起動:
```bash
go mod download
go run ./cmd/server
```

Goバックエンドは `http://localhost:8080` で起動します。

> **Note**: Django、Go、Jakarta EE、Rustのバックエンドは全て同じタスク管理APIを提供しています。フロントエンドの `NEXT_PUBLIC_API_URL` を切り替えることで、どのバックエンドも使用できます。Rustバックエンドにはお金の貸し借り管理APIも追加されています。

### バックエンド（Jakarta EE）

1. Jakarta EEバックエンドディレクトリに移動:
```bash
cd jakarta-backend
```

2. 開発サーバーを起動（簡単な方法）:
```bash
./dev.sh
```

または、手動で起動:
```bash
mvn clean package -DskipTests
mvn payara-micro:start
```

Jakarta EEバックエンドは `http://localhost:8081` で起動します。

### バックエンド（Rust）

1. Rustバックエンドディレクトリに移動:
```bash
cd rust-backend
```

2. 開発サーバーを起動（簡単な方法）:
```bash
./dev.sh
```

または、手動で起動:
```bash
cargo run
```

Rustバックエンドは `http://localhost:8082` で起動します。

**★ Rustバックエンドの追加機能: お金の貸し借り管理API**
- `GET /api/debts/` - 貸し借り一覧
- `POST /api/debts/` - 貸し借りを記録
- `POST /api/debts/:id/settle/` - 精算済みにする
- `GET /api/debts/summary/` - サマリー取得

### フロントエンド（Next.js）

1. フロントエンドディレクトリに移動:
```bash
cd frontend
```

2. 依存パッケージをインストール:
```bash
npm install
```

3. 開発サーバーを起動:
```bash
npm run dev
```

フロントエンドは `http://localhost:3000` で起動します。

## 使い方

1. ブラウザで `http://localhost:3000` にアクセス
2. 新規登録ページでアカウントを作成
3. ログイン後、タスク管理画面でタスクを追加・編集・削除できます

## デプロイ

本番環境へのデプロイ手順は [DEPLOYMENT.md](./DEPLOYMENT.md) を参照してください。

- **バックエンド**: Railway（PostgreSQL含む）
- **フロントエンド**: Vercel

詳細な手順とトラブルシューティングガイドが記載されています。

## プロジェクト構成

```
.
├── backend/                 # Djangoバックエンド
│   ├── config/             # Django設定
│   ├── authentication/     # 認証アプリ
│   ├── users/              # ユーザーアプリ
│   ├── tasks/              # タスクアプリ
│   └── manage.py
│
├── go-backend/             # Goバックエンド
│   ├── cmd/server/         # エントリーポイント
│   ├── internal/
│   │   ├── config/        # 設定管理
│   │   ├── database/      # DB接続
│   │   ├── handlers/      # APIハンドラー
│   │   ├── middleware/    # ミドルウェア
│   │   └── models/        # データモデル
│   ├── start.sh           # 起動スクリプト
│   └── dev.sh             # 開発用スクリプト
│
├── jakarta-backend/        # Jakarta EEバックエンド
│   ├── src/main/java/
│   │   └── com/taskapp/
│   │       ├── entity/    # JPAエンティティ
│   │       ├── resource/  # JAX-RSリソース
│   │       ├── service/   # ビジネスロジック
│   │       ├── repository/ # データアクセス
│   │       └── filter/    # フィルター
│   ├── start.sh           # 起動スクリプト
│   └── dev.sh             # 開発用スクリプト
│
├── rust-backend/           # Rustバックエンド ★新規追加
│   ├── src/
│   │   ├── handlers/      # APIハンドラー
│   │   ├── models/        # データモデル
│   │   ├── middleware/    # ミドルウェア
│   │   └── routes/        # ルーティング
│   ├── start.sh           # 起動スクリプト
│   └── dev.sh             # 開発用スクリプト
│
└── frontend/               # Next.jsフロントエンド
    ├── app/                # App Router
    │   ├── login/         # ログインページ
    │   ├── register/      # 登録ページ
    │   └── tasks/         # タスク管理ページ
    ├── components/         # UIコンポーネント
    ├── contexts/           # Reactコンテキスト
    └── lib/                # ユーティリティ
```

## データベース設計

### Userテーブル
Djangoの組み込みUserモデルを使用
- `id` - 主キー
- `username` - ユーザー名
- `email` - メールアドレス
- `password` - パスワード（ハッシュ化）

### Taskテーブル
- `id` - 主キー
- `title` - タスクタイトル
- `description` - タスク詳細
- `completed` - 完了フラグ
- `created_at` - 作成日時
- `updated_at` - 更新日時
- `user` - ユーザーID（外部キー）

### Debtテーブル（Rustバックエンドのみ）★新規
- `id` - 主キー
- `counterparty` - 相手の名前
- `amount` - 金額（円）
- `debt_type` - 種類（lent: 貸した / borrowed: 借りた）
- `description` - メモ・理由
- `is_settled` - 精算済みフラグ
- `created_at` - 作成日時
- `updated_at` - 更新日時
- `settled_at` - 精算日時
- `user` - ユーザーID（外部キー）
