# タスク管理アプリケーション

Next.js (フロントエンド) と Django (バックエンド) で作成したタスク管理アプリケーションです。

## 機能

- ユーザー新規登録
- ユーザーログイン
- タスクの追加
- タスクの編集
- タスクの削除
- タスクの完了状態の切り替え

## 技術スタック

### バックエンド
- Django 5.0.1
- Django REST Framework
- Token認証
- SQLite データベース
- CORS設定

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
