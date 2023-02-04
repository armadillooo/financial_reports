# financial_reports
日本企業の株価を取得可能なREST APIバックエンドサーバーのソースになります。

株価取得以外にもログイン機能やお気に入り登録機能などもあります。

# 使用技術
* 言語: Rust1.66
* FW: Axum0.6
* Postgresql
* Redis(セッション管理)
* OpenIDConnect(ログイン機能)

# アーキテクチャ
* ドメイン駆動設計
* レイヤードアーキテクチャ

# プロジェクト構成
プロジェクトは主に以下のフォルダで構成されています。
* domain: ドメインモデル層
  * アプリケーションの形式(Web, Desktop)によらないロジックをまとめる
* application: ユースケース層
  * ドメインモデルを利用して課題を解決する
* presentation: プレゼンテーション層
  * ユーザーに対する入出力を行う
    * JSONレスポンス
    * APIルーティング
  * Webアプリ固有の処理を行う
    * OpenIdConnectによる認証
    * セッション管理
* infrastructures:　インフラ層
  * DBなどのミドルウェアに関連する
  
ドメイン駆動設計のルールにより、domainを頂点として上位の層が下位の層に

依存しないようになっています。

また、各層の中はさらに機能ごとに以下のフォルダに分かれています。
* session: セッション管理
* auth: 認証機能
* company: 企業一覧取得
* stock: 株価取得
* user: ユーザー登録
* favorite: お気に入り登録
* portfolio: ポートフォリオ登録

# 機能一覧
以下のAPIが使用できます。

## API一覧
|URL|Http メソッド|機能|リクエストパラメータ|
|----|----|----|----|
|/api/auth/signin|Post|Googleアカウントによるユーザー新規登録|なし|
|/api/auth/login|Post|ログイン|なし|
|/api/auth/logout|Post|ログアウト|なし|
|/api/users/me|Get|自分のユーザー情報取得(ログイン中の場合)|なし|
|/api/users/me/favorites|Get|お気に入り一覧取得|なし|
|/api/users/me/favorites/{stock id}|Post|お気に入り登録|なし|
|/api/users/me/favorites/{stock id}|Delete|お気に入り削除|なし|
|/api/users/me/portfolio/{stock id}|Post|ポートフォリオ登録|なし|
|/api/users/me/portfolio/{stock id}|Delete|ポートフォリオ削除|なし|
|/api/users/me/portfolio/{stock id}|Patch|ポートフォリオ更新|stock_count: 購入株数<br>purchase: 購入価格|
|/api/companies|Get|企業情報取得|name： 企業名<br>stock_id: 証券コード<br>sector: セクター<br>industry: 産業|
|/api/stocks/{stock_id}|Get|株価情報取得|start: 開始日付<br>end: 終了日付<br>|

# 使用方法
## テストサーバーの起動
`cargo run --bin test_server`

## 本番用サーバーの起動
`cargo run --bin product_server`

テストサーバーを動かすためには以下の環境変数の設定が必要になります。(もしくは.envファイルに記載)

## 設定が必要な環境変数
* GOOGLE_CLIENT_ID: GoogleのクライアントID
* GOOGLE_CLIENT_SECRET: Googleのクライアントシークレット
* RUST_LOG: ログレベル(例 "INFO")
* SOCKET_ADDRESS: サーバーのアドレス(例 127.0.0.1:3000)

本番用サーバーを動かすためには上記に加えて以下の環境変数の設定が必要になります。
## 本番用サーバーの環境変数
* DATABASE_URL: PostgresqlのURL(例 "postgres://user_name:password@localhost:5432/db_name")
* SESSION_URL: RedisのURL(例 "redis://127.0.0.1")