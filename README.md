# freee-accounting-sdk-rust

### サンプルを実行するまでの手順
- 以下の環境変数に値をセットします
   - `RUST_API_SAMPLE_COMPANY_ID` 事業所ID（分からない場合は、examples/src/main.rsで最初にcompaniesエンドポイントをcallするのでその結果から選択して下さい）
   - `RUST_API_SAMPLE_BASE_PATH` APIエンドポイントのベースURL（通常は、`https://api.freee.co.jp`）
   - `RUST_API_SAMPLE_OAUTH_ACCESS_TOKEN` freeeアプリストアで得られるアクセストークン
- このREADME.mdがあるディレクトリで、`cargo run` を実行します。
- コンパイル後に、以下のような実行結果が出力されれば成功です。

    ```
    - company.id: <事業所ID>, company.display_name: <事業所の表示名>
    created: partner.id=<追加された取引先情報のID>, partner.name=Rust API SDKテスト
    got: partner.id=<追加された取引先情報のID>, partner.name=Rust API SDKテスト
    destroy: deleted the partner.
    ```
  