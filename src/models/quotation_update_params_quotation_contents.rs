/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。詳細は<a href=\"https://developer.freee.co.jp/docs\" target=\"_blank\">ドキュメントの認証</a>パートを参照してください。</p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p>  <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>レスポンスボディのmetaプロパティに以下を含めます。</p>  <ul>   <li>設定されている上限値</li>   <li>上限に達するまでの使用可能回数</li>   <li>（上限値に達した場合）使用回数がリセットされる時刻</li> </ul>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QuotationUpdateParamsQuotationContents {
    /// 見積内容ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 順序
    #[serde(rename = "order")]
    pub order: i32,
    /// 行の種類 <ul> <li>normal、discountを指定する場合、account_item_id,tax_codeとunit_priceが必須となります。</li> <li>normalを指定した場合、qtyが必須となります。</li> </ul>
    #[serde(rename = "type")]
    pub _type: Type,
    /// 数量
    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<f32>,
    /// 単位
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 単価 (tax_entry_method: inclusiveの場合は税込価格、tax_entry_method: exclusiveの場合は税抜価格となります)
    #[serde(rename = "unit_price", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f32>,
    /// 消費税額
    #[serde(rename = "vat", skip_serializing_if = "Option::is_none")]
    pub vat: Option<i32>,
    /// 備考
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勘定科目ID
    #[serde(rename = "account_item_id", skip_serializing_if = "Option::is_none")]
    pub account_item_id: Option<i32>,
    /// 税区分コード
    #[serde(rename = "tax_code", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<i32>,
    /// 品目ID
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    /// 部門ID
    #[serde(rename = "section_id", skip_serializing_if = "Option::is_none")]
    pub section_id: Option<i32>,
    #[serde(rename = "tag_ids", skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i32>>,
    /// セグメント１ID
    #[serde(rename = "segment_1_tag_id", skip_serializing_if = "Option::is_none")]
    pub segment_1_tag_id: Option<i64>,
    /// セグメント２ID
    #[serde(rename = "segment_2_tag_id", skip_serializing_if = "Option::is_none")]
    pub segment_2_tag_id: Option<i64>,
    /// セグメント３ID
    #[serde(rename = "segment_3_tag_id", skip_serializing_if = "Option::is_none")]
    pub segment_3_tag_id: Option<i64>,
}

impl QuotationUpdateParamsQuotationContents {
    pub fn new(order: i32, _type: Type) -> QuotationUpdateParamsQuotationContents {
        QuotationUpdateParamsQuotationContents {
            id: None,
            order,
            _type,
            qty: None,
            unit: None,
            unit_price: None,
            vat: None,
            description: None,
            account_item_id: None,
            tax_code: None,
            item_id: None,
            section_id: None,
            tag_ids: None,
            segment_1_tag_id: None,
            segment_2_tag_id: None,
            segment_3_tag_id: None,
        }
    }
}

/// 行の種類 <ul> <li>normal、discountを指定する場合、account_item_id,tax_codeとunit_priceが必須となります。</li> <li>normalを指定した場合、qtyが必須となります。</li> </ul>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "discount")]
    Discount,
    #[serde(rename = "text")]
    Text,
}

