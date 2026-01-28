# Jakarta EE Backend

タスク管理APIのJakarta EEバックエンド実装です。Django REST FrameworkおよびGoで実装された既存のバックエンドと同じ機能・APIを提供します。

## 技術スタック

- **Java**: 17+
- **Jakarta EE**: 10
- **JAX-RS**: RESTful API
- **JPA**: データベースアクセス (Hibernate)
- **CDI**: 依存性注入
- **Bean Validation**: 入力バリデーション
- **サーバー**: Payara Micro
- **データベース**: H2 (開発) / PostgreSQL (本番)

## プロジェクト構造

```
jakarta-backend/
├── src/main/java/com/taskapp/
│   ├── config/
│   │   └── JaxRsApplication.java    # JAX-RS設定
│   ├── entity/
│   │   ├── User.java                # ユーザーエンティティ
│   │   ├── Task.java                # タスクエンティティ
│   │   └── Token.java               # トークンエンティティ
│   ├── dto/
│   │   ├── UserDTO.java             # ユーザーDTO
│   │   ├── TaskDTO.java             # タスクDTO
│   │   ├── AuthDTO.java             # 認証DTO
│   │   └── ErrorDTO.java            # エラーDTO
│   ├── repository/
│   │   ├── UserRepository.java      # ユーザーリポジトリ
│   │   ├── TaskRepository.java      # タスクリポジトリ
│   │   └── TokenRepository.java     # トークンリポジトリ
│   ├── service/
│   │   ├── AuthService.java         # 認証サービス
│   │   └── TaskService.java         # タスクサービス
│   ├── resource/
│   │   ├── AuthResource.java        # 認証API
│   │   ├── UserResource.java        # ユーザーAPI
│   │   ├── TaskResource.java        # タスクAPI
│   │   └── HealthResource.java      # ヘルスチェック
│   ├── filter/
│   │   ├── CorsFilter.java          # CORSフィルター
│   │   ├── AuthFilter.java          # 認証フィルター
│   │   └── Authenticated.java       # 認証アノテーション
│   └── util/
│       ├── PasswordUtil.java        # パスワードハッシュ
│       └── TokenUtil.java           # トークン生成
├── src/main/resources/
│   └── META-INF/
│       └── persistence.xml          # JPA設定
├── src/main/webapp/
│   └── WEB-INF/
│       └── beans.xml                # CDI設定
├── pom.xml                          # Maven設定
├── start.sh                         # 起動スクリプト
├── dev.sh                           # 開発用スクリプト
└── README.md
```

## 前提条件

- **Java 17** 以上
- **Maven 3.8** 以上

### インストール

**macOS:**
```bash
brew install openjdk@17 maven
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install openjdk-17-jdk maven
```

## クイックスタート

```bash
cd jakarta-backend

# 開発サーバー起動（推奨）
./dev.sh

# または、手動でビルドして起動
mvn clean package -DskipTests
mvn payara-micro:start
```

サーバーは `http://localhost:8081` で起動します。

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
| GET | `/api/health` | 不要 | サーバーの状態確認 |

## 認証の使い方

### 1. ユーザー登録

```bash
curl -X POST http://localhost:8081/api/auth/register \
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

### 2. 認証が必要なAPIへのアクセス

```bash
curl -X GET http://localhost:8081/api/tasks/ \
  -H "Authorization: Token abc123..."
```

## 環境変数

| 変数名 | デフォルト | 説明 |
|--------|-----------|------|
| `CORS_ALLOWED_ORIGINS` | `http://localhost:3000` | 許可するオリジン（カンマ区切り） |

## 他のバックエンドとの互換性

このJakarta EEバックエンドは、既存のDjango/Goバックエンドと完全に互換性があります：

- 同じAPIエンドポイント構造
- 同じリクエスト/レスポンス形式
- 同じトークン認証方式 (`Authorization: Token <key>`)
- 同じエラーレスポンス形式

フロントエンドのAPI URLを変更するだけで、Django/Go/Jakarta EEを切り替えることができます。

## アーキテクチャ

### レイヤー構成

```
Resource (JAX-RS)
    ↓
Service (ビジネスロジック)
    ↓
Repository (データアクセス)
    ↓
Entity (JPA)
```

### ベストプラクティス

- **セキュリティ**: パスワードはPBKDF2でハッシュ化
- **トランザクション**: JPAトランザクション管理
- **バリデーション**: Bean Validationによる入力検証
- **依存性注入**: CDIによるDI
- **フィルター**: JAX-RSフィルターによる認証

## 本番環境へのデプロイ

```bash
# 環境変数を設定
export CORS_ALLOWED_ORIGINS="https://your-frontend.com"

# WARファイルをビルド
mvn clean package -Pprod

# Payara ServerやWildFlyにデプロイ
# または、Payara Microで起動
java -jar payara-micro.jar --deploy target/taskapp.war
```

## トラブルシューティング

### ポート8081が使用中の場合

pom.xmlの`payara-micro-maven-plugin`設定でポートを変更：

```xml
<option>
    <key>--port</key>
    <value>8082</value>
</option>
```

### データベースをリセットする場合

`data/`ディレクトリを削除してサーバーを再起動：

```bash
rm -rf data/
./dev.sh
```

## ライセンス

MIT License
