/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">お知らせ</h2>  <p> <b>インボイス制度に伴い、freee会計の帳票機能がfreee請求書に移行します。これに伴い、2023年10月にfreee会計の「請求書の作成、見積書の作成」エンドポイントは廃止、freee請求書APIに移行する予定です。詳細は<a href=\"https://developer.freee.co.jp/news/6369\" target=\"_blank\"> freee会計 APIの仕様変更（インボイス制度対応）について</a>をご確認ください。</b> </p>  <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> <li>エラーメッセージの変更</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUsersCapabilities200Response {
    #[serde(rename = "wallet_txns")]
    pub wallet_txns: Box<crate::models::UserCapabilityWithConfirm>,
    #[serde(rename = "deals")]
    pub deals: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "transfers")]
    pub transfers: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "docs")]
    pub docs: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "doc_postings")]
    pub doc_postings: Box<crate::models::UserCapabilityJustCreate>,
    #[serde(rename = "receipts")]
    pub receipts: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "receipt_stream_editor")]
    pub receipt_stream_editor: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "spreadsheets")]
    pub spreadsheets: Box<crate::models::UserCapabilityJustCreateRead>,
    #[serde(rename = "expense_applications")]
    pub expense_applications: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "expense_application_sync_payroll")]
    pub expense_application_sync_payroll: Box<crate::models::UserCapabilityJustCreate>,
    #[serde(rename = "payment_requests")]
    pub payment_requests: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "approval_requests")]
    pub approval_requests: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "reports")]
    pub reports: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_income_expense")]
    pub reports_income_expense: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_receivables")]
    pub reports_receivables: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_payables")]
    pub reports_payables: Box<crate::models::UserCapabilityJustReadWrite>,
    #[serde(rename = "reports_cash_balance")]
    pub reports_cash_balance: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_managements_planning")]
    pub reports_managements_planning: Box<crate::models::UserCapabilityJustReadWrite>,
    #[serde(rename = "reports_managements_navigation")]
    pub reports_managements_navigation: Box<crate::models::UserCapabilityJustReadWrite>,
    #[serde(rename = "reports_crosstabs")]
    pub reports_crosstabs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_custom_reports_aggregate")]
    pub reports_custom_reports_aggregate: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_pl")]
    pub reports_pl: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_bs")]
    pub reports_bs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_general_ledgers")]
    pub reports_general_ledgers: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "reports_journals")]
    pub reports_journals: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "manual_journals")]
    pub manual_journals: Box<crate::models::UserCapabilityWithSelfOnly>,
    #[serde(rename = "fixed_assets")]
    pub fixed_assets: Box<crate::models::UserCapability>,
    #[serde(rename = "inventory_refreshes")]
    pub inventory_refreshes: Box<crate::models::UserCapability>,
    #[serde(rename = "biz_allocations")]
    pub biz_allocations: Box<crate::models::UserCapability>,
    #[serde(rename = "payment_records")]
    pub payment_records: Box<crate::models::UserCapability>,
    #[serde(rename = "annual_reports")]
    pub annual_reports: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "tax_reports")]
    pub tax_reports: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "consumption_entries")]
    pub consumption_entries: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "tax_return")]
    pub tax_return: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "account_item_statements")]
    pub account_item_statements: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "month_end")]
    pub month_end: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "year_end")]
    pub year_end: Box<crate::models::UserCapabilityJustReadUpdate>,
    #[serde(rename = "walletables")]
    pub walletables: Box<crate::models::UserCapabilityWithSync>,
    #[serde(rename = "companies")]
    pub companies: Box<crate::models::UserCapabilityJustReadUpdate>,
    #[serde(rename = "invitations")]
    pub invitations: Box<crate::models::UserCapability>,
    #[serde(rename = "access_controls")]
    pub access_controls: Box<crate::models::UserCapabilityWithWrite>,
    #[serde(rename = "sign_in_logs")]
    pub sign_in_logs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "user_attribute_logs")]
    pub user_attribute_logs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "app_role_logs")]
    pub app_role_logs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "txn_relationship_logs")]
    pub txn_relationship_logs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "backups")]
    pub backups: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "opening_balances")]
    pub opening_balances: Box<crate::models::UserCapabilityJustReadUpdate>,
    #[serde(rename = "system_conversion")]
    pub system_conversion: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "resets")]
    pub resets: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "partners")]
    pub partners: Box<crate::models::UserCapability>,
    #[serde(rename = "items")]
    pub items: Box<crate::models::UserCapability>,
    #[serde(rename = "sections")]
    pub sections: Box<crate::models::UserCapability>,
    #[serde(rename = "tags")]
    pub tags: Box<crate::models::UserCapability>,
    #[serde(rename = "account_items")]
    pub account_items: Box<crate::models::UserCapability>,
    #[serde(rename = "taxes")]
    pub taxes: Box<crate::models::UserCapabilityJustReadUpdate>,
    #[serde(rename = "payroll_item_sets")]
    pub payroll_item_sets: Box<crate::models::UserCapability>,
    #[serde(rename = "user_matchers")]
    pub user_matchers: Box<crate::models::UserCapability>,
    #[serde(rename = "deal_templates")]
    pub deal_templates: Box<crate::models::UserCapability>,
    #[serde(rename = "manual_journal_templates")]
    pub manual_journal_templates: Box<crate::models::UserCapability>,
    #[serde(rename = "cost_allocations")]
    pub cost_allocations: Box<crate::models::UserCapabilityJustReadUpdate>,
    #[serde(rename = "approval_flow_routes")]
    pub approval_flow_routes: Box<crate::models::UserCapability>,
    #[serde(rename = "expense_application_templates")]
    pub expense_application_templates: Box<crate::models::UserCapability>,
    #[serde(rename = "request_forms")]
    pub request_forms: Box<crate::models::UserCapability>,
    #[serde(rename = "system_messages_for_admin")]
    pub system_messages_for_admin: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "company_internal_announcements")]
    pub company_internal_announcements: Box<crate::models::UserCapabilityJustUpdate>,
    #[serde(rename = "doc_change_logs")]
    pub doc_change_logs: Box<crate::models::UserCapabilityJustRead>,
    #[serde(rename = "workflows")]
    pub workflows: Box<crate::models::UserCapabilityJustReadUpdateDestroy>,
    #[serde(rename = "oauth_applications")]
    pub oauth_applications: Box<crate::models::UserCapability>,
    #[serde(rename = "oauth_authorizations")]
    pub oauth_authorizations: Box<crate::models::UserCapability>,
    #[serde(rename = "bank_accountant_staff_users")]
    pub bank_accountant_staff_users: Box<crate::models::UserCapability>,
}

impl GetUsersCapabilities200Response {
    pub fn new(wallet_txns: crate::models::UserCapabilityWithConfirm, deals: crate::models::UserCapabilityWithSelfOnly, transfers: crate::models::UserCapabilityWithSelfOnly, docs: crate::models::UserCapabilityWithSelfOnly, doc_postings: crate::models::UserCapabilityJustCreate, receipts: crate::models::UserCapabilityWithSelfOnly, receipt_stream_editor: crate::models::UserCapabilityJustRead, spreadsheets: crate::models::UserCapabilityJustCreateRead, expense_applications: crate::models::UserCapabilityWithSelfOnly, expense_application_sync_payroll: crate::models::UserCapabilityJustCreate, payment_requests: crate::models::UserCapabilityWithSelfOnly, approval_requests: crate::models::UserCapabilityWithSelfOnly, reports: crate::models::UserCapabilityJustRead, reports_income_expense: crate::models::UserCapabilityJustRead, reports_receivables: crate::models::UserCapabilityJustRead, reports_payables: crate::models::UserCapabilityJustReadWrite, reports_cash_balance: crate::models::UserCapabilityJustRead, reports_managements_planning: crate::models::UserCapabilityJustReadWrite, reports_managements_navigation: crate::models::UserCapabilityJustReadWrite, reports_crosstabs: crate::models::UserCapabilityJustRead, reports_custom_reports_aggregate: crate::models::UserCapabilityJustRead, reports_pl: crate::models::UserCapabilityJustRead, reports_bs: crate::models::UserCapabilityJustRead, reports_general_ledgers: crate::models::UserCapabilityJustRead, reports_journals: crate::models::UserCapabilityJustRead, manual_journals: crate::models::UserCapabilityWithSelfOnly, fixed_assets: crate::models::UserCapability, inventory_refreshes: crate::models::UserCapability, biz_allocations: crate::models::UserCapability, payment_records: crate::models::UserCapability, annual_reports: crate::models::UserCapabilityJustRead, tax_reports: crate::models::UserCapabilityJustRead, consumption_entries: crate::models::UserCapabilityJustRead, tax_return: crate::models::UserCapabilityJustRead, account_item_statements: crate::models::UserCapabilityJustRead, month_end: crate::models::UserCapabilityJustRead, year_end: crate::models::UserCapabilityJustReadUpdate, walletables: crate::models::UserCapabilityWithSync, companies: crate::models::UserCapabilityJustReadUpdate, invitations: crate::models::UserCapability, access_controls: crate::models::UserCapabilityWithWrite, sign_in_logs: crate::models::UserCapabilityJustRead, user_attribute_logs: crate::models::UserCapabilityJustRead, app_role_logs: crate::models::UserCapabilityJustRead, txn_relationship_logs: crate::models::UserCapabilityJustRead, backups: crate::models::UserCapabilityJustRead, opening_balances: crate::models::UserCapabilityJustReadUpdate, system_conversion: crate::models::UserCapabilityJustRead, resets: crate::models::UserCapabilityJustRead, partners: crate::models::UserCapability, items: crate::models::UserCapability, sections: crate::models::UserCapability, tags: crate::models::UserCapability, account_items: crate::models::UserCapability, taxes: crate::models::UserCapabilityJustReadUpdate, payroll_item_sets: crate::models::UserCapability, user_matchers: crate::models::UserCapability, deal_templates: crate::models::UserCapability, manual_journal_templates: crate::models::UserCapability, cost_allocations: crate::models::UserCapabilityJustReadUpdate, approval_flow_routes: crate::models::UserCapability, expense_application_templates: crate::models::UserCapability, request_forms: crate::models::UserCapability, system_messages_for_admin: crate::models::UserCapabilityJustRead, company_internal_announcements: crate::models::UserCapabilityJustUpdate, doc_change_logs: crate::models::UserCapabilityJustRead, workflows: crate::models::UserCapabilityJustReadUpdateDestroy, oauth_applications: crate::models::UserCapability, oauth_authorizations: crate::models::UserCapability, bank_accountant_staff_users: crate::models::UserCapability) -> GetUsersCapabilities200Response {
        GetUsersCapabilities200Response {
            wallet_txns: Box::new(wallet_txns),
            deals: Box::new(deals),
            transfers: Box::new(transfers),
            docs: Box::new(docs),
            doc_postings: Box::new(doc_postings),
            receipts: Box::new(receipts),
            receipt_stream_editor: Box::new(receipt_stream_editor),
            spreadsheets: Box::new(spreadsheets),
            expense_applications: Box::new(expense_applications),
            expense_application_sync_payroll: Box::new(expense_application_sync_payroll),
            payment_requests: Box::new(payment_requests),
            approval_requests: Box::new(approval_requests),
            reports: Box::new(reports),
            reports_income_expense: Box::new(reports_income_expense),
            reports_receivables: Box::new(reports_receivables),
            reports_payables: Box::new(reports_payables),
            reports_cash_balance: Box::new(reports_cash_balance),
            reports_managements_planning: Box::new(reports_managements_planning),
            reports_managements_navigation: Box::new(reports_managements_navigation),
            reports_crosstabs: Box::new(reports_crosstabs),
            reports_custom_reports_aggregate: Box::new(reports_custom_reports_aggregate),
            reports_pl: Box::new(reports_pl),
            reports_bs: Box::new(reports_bs),
            reports_general_ledgers: Box::new(reports_general_ledgers),
            reports_journals: Box::new(reports_journals),
            manual_journals: Box::new(manual_journals),
            fixed_assets: Box::new(fixed_assets),
            inventory_refreshes: Box::new(inventory_refreshes),
            biz_allocations: Box::new(biz_allocations),
            payment_records: Box::new(payment_records),
            annual_reports: Box::new(annual_reports),
            tax_reports: Box::new(tax_reports),
            consumption_entries: Box::new(consumption_entries),
            tax_return: Box::new(tax_return),
            account_item_statements: Box::new(account_item_statements),
            month_end: Box::new(month_end),
            year_end: Box::new(year_end),
            walletables: Box::new(walletables),
            companies: Box::new(companies),
            invitations: Box::new(invitations),
            access_controls: Box::new(access_controls),
            sign_in_logs: Box::new(sign_in_logs),
            user_attribute_logs: Box::new(user_attribute_logs),
            app_role_logs: Box::new(app_role_logs),
            txn_relationship_logs: Box::new(txn_relationship_logs),
            backups: Box::new(backups),
            opening_balances: Box::new(opening_balances),
            system_conversion: Box::new(system_conversion),
            resets: Box::new(resets),
            partners: Box::new(partners),
            items: Box::new(items),
            sections: Box::new(sections),
            tags: Box::new(tags),
            account_items: Box::new(account_items),
            taxes: Box::new(taxes),
            payroll_item_sets: Box::new(payroll_item_sets),
            user_matchers: Box::new(user_matchers),
            deal_templates: Box::new(deal_templates),
            manual_journal_templates: Box::new(manual_journal_templates),
            cost_allocations: Box::new(cost_allocations),
            approval_flow_routes: Box::new(approval_flow_routes),
            expense_application_templates: Box::new(expense_application_templates),
            request_forms: Box::new(request_forms),
            system_messages_for_admin: Box::new(system_messages_for_admin),
            company_internal_announcements: Box::new(company_internal_announcements),
            doc_change_logs: Box::new(doc_change_logs),
            workflows: Box::new(workflows),
            oauth_applications: Box::new(oauth_applications),
            oauth_authorizations: Box::new(oauth_authorizations),
            bank_accountant_staff_users: Box::new(bank_accountant_staff_users),
        }
    }
}


