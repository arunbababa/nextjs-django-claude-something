# デプロイ手順

このガイドでは、バックエンド（Django）をRailwayに、フロントエンド（Next.js）をVercelにデプロイする手順を説明します。

## 目次
1. [バックエンド（Django）のデプロイ - Railway](#バックエンドdjangoのデプロイ---railway)
2. [フロントエンド（Next.js）のデプロイ - Vercel](#フロントエンドnextjsのデプロイ---vercel)
3. [デプロイ後の設定](#デプロイ後の設定)

---

## バックエンド（Django）のデプロイ - Railway

### 前提条件
- GitHubアカウント
- Railwayアカウント（https://railway.app/ で無料登録）

### 手順

#### 1. Railwayにログイン
1. https://railway.app/ にアクセス
2. 「Login」をクリックしてGitHubアカウントでログイン

#### 2. 新しいプロジェクトを作成
1. ダッシュボードで「New Project」をクリック
2. 「Deploy from GitHub repo」を選択
3. リポジトリを選択（このプロジェクトのリポジトリ）
4. 「Deploy Now」をクリック

#### 3. PostgreSQLデータベースを追加
1. プロジェクトダッシュボードで「New」→「Database」→「Add PostgreSQL」をクリック
2. データベースが自動的に作成され、環境変数`DATABASE_URL`が設定されます

#### 4. 環境変数を設定
1. プロジェクトダッシュボードで「Variables」タブを選択
2. 以下の環境変数を追加：

```
SECRET_KEY=ランダムな文字列（50文字以上推奨）
DEBUG=False
ALLOWED_HOSTS=your-app-name.railway.app
CORS_ALLOWED_ORIGINS=https://your-frontend-domain.vercel.app
```

**SECRET_KEYの生成方法**:
```python
# Pythonで実行
import secrets
print(secrets.token_urlsafe(50))
```

#### 5. ルートディレクトリを設定
1. 「Settings」タブを選択
2. 「Root Directory」を`backend`に設定
3. 「Deploy」をクリック

#### 6. デプロイ完了を確認
1. 「Deployments」タブでビルドログを確認
2. デプロイが成功したら、「Settings」→「Networking」→「Generate Domain」をクリック
3. 生成されたURLをメモ（例: `https://your-app-name.railway.app`）

#### 7. 動作確認
ブラウザで以下のURLにアクセスして確認：
```
https://your-app-name.railway.app/api/auth/login
```
→ JSONレスポンスが返ってくればOK

---

## フロントエンド（Next.js）のデプロイ - Vercel

### 前提条件
- GitHubアカウント
- Vercelアカウント（https://vercel.com で無料登録）

### 手順

#### 1. Vercelにログイン
1. https://vercel.com にアクセス
2. 「Sign Up」をクリックしてGitHubアカウントでログイン

#### 2. 新しいプロジェクトをインポート
1. ダッシュボードで「Add New...」→「Project」をクリック
2. GitHubリポジトリを選択
3. 「Import」をクリック

#### 3. プロジェクト設定
1. **Framework Preset**: `Next.js`（自動検出されます）
2. **Root Directory**: `frontend`を選択
3. **Build Command**: デフォルトのまま（`npm run build`）
4. **Output Directory**: デフォルトのまま（`.next`）

#### 4. 環境変数を設定
「Environment Variables」セクションで以下を追加：

```
NEXT_PUBLIC_API_URL=https://your-backend-name.railway.app/api
```

**重要**: `your-backend-name.railway.app`を実際のRailwayのURLに置き換えてください。

#### 5. デプロイ
1. 「Deploy」をクリック
2. デプロイが完了するまで待機（通常1〜3分）

#### 6. デプロイ完了を確認
1. デプロイが完了したら、「Visit」をクリック
2. 生成されたURL（例: `https://your-app-name.vercel.app`）をメモ

---

## デプロイ後の設定

### 1. バックエンドのCORS設定を更新

Railwayのダッシュボードで環境変数`CORS_ALLOWED_ORIGINS`を更新：

```
CORS_ALLOWED_ORIGINS=https://your-frontend-name.vercel.app
```

変更後、Railwayで再デプロイ（自動的に再デプロイされる場合もあります）

### 2. 動作確認

1. Vercelで生成されたURL（例: `https://your-app-name.vercel.app`）にアクセス
2. 新規登録ページでアカウントを作成
3. ログイン
4. タスクを追加・編集・削除してみる

### 3. カスタムドメインの設定（オプション）

#### Vercel（フロントエンド）
1. プロジェクト設定→「Domains」
2. カスタムドメインを追加
3. DNSレコードを設定

#### Railway（バックエンド）
1. プロジェクト設定→「Settings」→「Networking」
2. 「Custom Domain」を追加
3. DNSレコードを設定

カスタムドメインを設定した場合、環境変数を更新：
- RailwayのALLOWED_HOSTSとCORS_ALLOWED_ORIGINS
- VercelのNEXT_PUBLIC_API_URL

---

## トラブルシューティング

### バックエンドがデプロイできない場合

1. **ビルドログを確認**: Railwayの「Deployments」タブでエラーメッセージを確認
2. **環境変数の確認**: すべての必要な環境変数が設定されているか確認
3. **ルートディレクトリの確認**: `backend`に設定されているか確認

### フロントエンドがデプロイできない場合

1. **ビルドログを確認**: Vercelのデプロイログでエラーを確認
2. **ルートディレクトリの確認**: `frontend`に設定されているか確認
3. **環境変数の確認**: NEXT_PUBLIC_API_URLが正しく設定されているか確認

### CORS エラーが発生する場合

1. RailwayのCORS_ALLOWED_ORIGINSにVercelのURLが正しく設定されているか確認
2. プロトコル（https）が含まれているか確認
3. 末尾にスラッシュ（/）がないことを確認

### データベース接続エラー

1. PostgreSQLデータベースが追加されているか確認
2. DATABASE_URL環境変数が自動的に設定されているか確認

---

## コスト

### Railway
- 無料枠: $5 クレジット/月（Hobby Plan）
- PostgreSQL含む
- 十分な小規模アプリには無料枠で対応可能

### Vercel
- 無料枠: Hobby Plan
- 商用利用可能
- カスタムドメイン対応

---

## まとめ

デプロイが完了すると、以下のようなURLが生成されます：

- **フロントエンド**: https://your-app-name.vercel.app
- **バックエンド**: https://your-backend-name.railway.app

これでタスク管理アプリケーションが本番環境で動作します！
