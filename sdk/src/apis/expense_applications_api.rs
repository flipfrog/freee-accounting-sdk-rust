/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">お知らせ</h2>  <p> <b>インボイス制度に伴い、freee会計の帳票機能がfreee請求書に移行します。これに伴い、2023年10月にfreee会計の「請求書の作成、見積書の作成」エンドポイントは廃止、freee請求書APIに移行する予定です。詳細は<a href=\"https://developer.freee.co.jp/news/6369\" target=\"_blank\"> freee会計 APIの仕様変更（インボイス制度対応）について</a>をご確認ください。</b> </p>  <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_expense_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExpenseApplicationError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`destroy_expense_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DestroyExpenseApplicationError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_expense_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExpenseApplicationError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_expense_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExpenseApplicationsError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_expense_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateExpenseApplicationError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_expense_application_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateExpenseApplicationActionError {
    Status400(crate::models::BadRequestError),
    Status401(crate::models::UnauthorizedError),
    Status403(crate::models::ForbiddenError),
    Status404(crate::models::BadRequestNotFoundError),
    Status500(crate::models::InternalServerError),
    UnknownValue(serde_json::Value),
}


///  <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請を作成する</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請は本API経由で作成ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>申請時には、申請タイトル(title)に加え、項目行については金額(amount)、日付(transaction_date)、内容(description)が必須項目となります。申請時の業務効率化のため、API入力をお勧めします。</li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は作成できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>  <pre> <code>インボイス制度の対応にともない明細行のデータ構造が変更されます。<br> 新しいデータ構造は2023年5月下旬から利用できる予定です。利用するにはリクエストヘッダーに以下を指定します。 Expense-Application-Mode: purchase-lines<br> 指定がない場合は従来のデータ構造を利用することとなります。<br> 詳細は、<a href=\"https://developer.freee.co.jp/news/6369#2-%e7%b5%8c%e8%b2%bb%e7%b2%be%e7%ae%97api%e3%81%ae%e4%bb%95%e6%a7%98%e5%a4%89%e6%9b%b4%ef%bc%88%e7%a0%b4%e5%a3%8a%e7%9a%84%e5%a4%89%e6%9b%b4%ef%bc%89\" target=\"_blank\">お知らせ</a>をご覧ください。</code> </pre>
pub async fn create_expense_application(configuration: &configuration::Configuration, expense_application_create_params: Option<crate::models::ExpenseApplicationCreateParams>) -> Result<crate::models::ExpenseApplicationResponse, Error<CreateExpenseApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&expense_application_create_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateExpenseApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を削除する</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>       <li>自分が申請者でない申請の削除が可能なのはユーザーの権限が管理者権限、且つ申請ステータスが差し戻しの場合のみです</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は削除できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>
pub async fn destroy_expense_application(configuration: &configuration::Configuration, id: i32, company_id: i32) -> Result<(), Error<DestroyExpenseApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DestroyExpenseApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を取得する</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は取得できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>  <pre> <code>インボイス制度の対応にともない明細行のデータ構造が変更されます。<br> 新しいデータ構造は2023年5月下旬から利用できる予定です。利用するにはリクエストヘッダーに以下を指定します。 Expense-Application-Mode: purchase-lines<br> 指定がない場合は従来のデータ構造を利用することとなります。<br> 詳細は、<a href=\"https://developer.freee.co.jp/news/6369#2-%e7%b5%8c%e8%b2%bb%e7%b2%be%e7%ae%97api%e3%81%ae%e4%bb%95%e6%a7%98%e5%a4%89%e6%9b%b4%ef%bc%88%e7%a0%b4%e5%a3%8a%e7%9a%84%e5%a4%89%e6%9b%b4%ef%bc%89\" target=\"_blank\">お知らせ</a>をご覧ください。</code> </pre>
pub async fn get_expense_application(configuration: &configuration::Configuration, id: i32, company_id: i32) -> Result<crate::models::ExpenseApplicationResponse, Error<GetExpenseApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetExpenseApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請一覧を取得する</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請の一覧を取得することができます。</li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>  <pre> <code>インボイス制度の対応にともない明細行のデータ構造が変更されます。<br> 新しいデータ構造は2023年5月下旬から利用できる予定です。利用するにはリクエストヘッダーに以下を指定します。 Expense-Application-Mode: purchase-lines<br> 指定がない場合は従来のデータ構造を利用することとなります。<br> 詳細は、<a href=\"https://developer.freee.co.jp/news/6369#2-%e7%b5%8c%e8%b2%bb%e7%b2%be%e7%ae%97api%e3%81%ae%e4%bb%95%e6%a7%98%e5%a4%89%e6%9b%b4%ef%bc%88%e7%a0%b4%e5%a3%8a%e7%9a%84%e5%a4%89%e6%9b%b4%ef%bc%89\" target=\"_blank\">お知らせ</a>をご覧ください。</code> </pre>
pub async fn get_expense_applications(configuration: &configuration::Configuration, company_id: i32, status: Option<&str>, payroll_attached: Option<bool>, start_transaction_date: Option<&str>, end_transaction_date: Option<&str>, application_number: Option<i32>, title: Option<&str>, start_issue_date: Option<&str>, end_issue_date: Option<&str>, applicant_id: Option<i32>, approver_id: Option<i32>, min_amount: Option<i32>, max_amount: Option<i32>, offset: Option<i64>, limit: Option<i32>) -> Result<crate::models::ExpenseApplicationsIndexResponse, Error<GetExpenseApplicationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("company_id", &company_id.to_string())]);
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = payroll_attached {
        local_var_req_builder = local_var_req_builder.query(&[("payroll_attached", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_transaction_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_transaction_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_transaction_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_transaction_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = application_number {
        local_var_req_builder = local_var_req_builder.query(&[("application_number", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = title {
        local_var_req_builder = local_var_req_builder.query(&[("title", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_issue_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_issue_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_issue_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_issue_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = applicant_id {
        local_var_req_builder = local_var_req_builder.query(&[("applicant_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = approver_id {
        local_var_req_builder = local_var_req_builder.query(&[("approver_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_amount {
        local_var_req_builder = local_var_req_builder.query(&[("min_amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_amount {
        local_var_req_builder = local_var_req_builder.query(&[("max_amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetExpenseApplicationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を更新する</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請を更新することができます。</li>   <li>本APIでは、status(申請ステータス): draft:下書き, feedback:差戻しのみ更新可能です。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請は本API経由で更新ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>申請時には、申請タイトル(title)に加え、項目行については金額(amount)、日付(transaction_date)、内容(description)が必須項目となります。申請時の業務効率化のため、API入力をお勧めします。</li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は更新できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>  <pre> <code>インボイス制度の対応にともない明細行のデータ構造が変更されます。<br> 新しいデータ構造は2023年5月下旬から利用できる予定です。利用するにはリクエストヘッダーに以下を指定します。 Expense-Application-Mode: purchase-lines<br> 指定がない場合は従来のデータ構造を利用することとなります。<br> 詳細は、<a href=\"https://developer.freee.co.jp/news/6369#2-%e7%b5%8c%e8%b2%bb%e7%b2%be%e7%ae%97api%e3%81%ae%e4%bb%95%e6%a7%98%e5%a4%89%e6%9b%b4%ef%bc%88%e7%a0%b4%e5%a3%8a%e7%9a%84%e5%a4%89%e6%9b%b4%ef%bc%89\" target=\"_blank\">お知らせ</a>をご覧ください。</code> </pre>
pub async fn update_expense_application(configuration: &configuration::Configuration, id: i32, expense_application_update_params: Option<crate::models::ExpenseApplicationUpdateParams>) -> Result<crate::models::ExpenseApplicationResponse, Error<UpdateExpenseApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&expense_application_update_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateExpenseApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

///  <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請の承認操作を行う</p>  <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請の承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）を行うことができます。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li> 　<li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請はAPI経由で承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は承認操作できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>  <pre> <code>インボイス制度の対応にともない明細行のデータ構造が変更されます。<br> 新しいデータ構造は2023年5月下旬から利用できる予定です。利用するにはリクエストヘッダーに以下を指定します。 Expense-Application-Mode: purchase-lines<br> 指定がない場合は従来のデータ構造を利用することとなります。<br> 詳細は、<a href=\"https://developer.freee.co.jp/news/6369#2-%e7%b5%8c%e8%b2%bb%e7%b2%be%e7%ae%97api%e3%81%ae%e4%bb%95%e6%a7%98%e5%a4%89%e6%9b%b4%ef%bc%88%e7%a0%b4%e5%a3%8a%e7%9a%84%e5%a4%89%e6%9b%b4%ef%bc%89\" target=\"_blank\">お知らせ</a>をご覧ください。</code> </pre>
pub async fn update_expense_application_action(configuration: &configuration::Configuration, id: i32, expense_application_action_create_params: crate::models::ExpenseApplicationActionCreateParams) -> Result<crate::models::ExpenseApplicationResponse, Error<UpdateExpenseApplicationActionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/1/expense_applications/{id}/actions", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&expense_application_action_create_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateExpenseApplicationActionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

