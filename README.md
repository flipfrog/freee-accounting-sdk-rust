# freee-accounting-sdk-rust

## サンプルを実行するまでの手順
### コンソールで実行
- 以下の環境変数に値をセットします
   - `RUST_API_EXAMPLE_COMPANY_ID` 事業所ID（分からない場合は、examples/src/main.rsで最初にcompaniesエンドポイントをcallするのでその結果から選択して下さい）
   - `RUST_API_EXAMPLE_OAUTH_ACCESS_TOKEN` freeeアプリストアで得られるアクセストークン
- examples/consoleディレクトリで、`cargo run` を実行します。
- コンパイル後に、以下のような実行結果が出力されれば成功です。

    ```
    - company.id: <事業所ID>, company.display_name: <事業所の表示名>
    created: partner.id=<追加された取引先情報のID>, partner.name=Rust API SDKテスト
    got: partner.id=<追加された取引先情報のID>, partner.name=Rust API SDKテスト
    destroy: deleted the partner.
    ```
### webサーバとして実行
- freeeアプリストアでの設定
  - アプリ設定のコールバックURLを、`http://localhost:8080/auth_callback` に設定保存します。
  - コールバックURLのホスト名は、サンプルのwebサーバーが動くホスト名にします。
- 以下の環境変数に値をセットします
  - `RUST_API_EXAMPLE_CLIENT_ID`: アプリ設定の、CLIENT IDをコピーしてセットします。
  - `RUST_API_EXAMPLE_CLIENT_SECRET`: アプリ設定の、CLIENT SECRETをコピーしてセットします。
- examples/webディレクトリで、`cargo run` を実行します。
  - ブラウザで、http://localhost:8080/ にアクセスします。
  - 「freeeで認証する」リンクをクリックします。
  - freeeにログインしていない場合は、ログイン画面が表示されるので、ログインします。
  - アプリ連携の開始画面が表示されるので、「許可する」をクリックします。
  - APIを使った通信をして、「ログインしました」画面が表示され、ユーザ名と、事業所情報が表示されます。
