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
pub struct CompanyResponseCompany {
    /// 事業所ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 事業所の正式名称 (100文字以内)
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// 正式名称フリガナ (100文字以内)
    #[serde(rename = "name_kana")]
    pub name_kana: Option<String>,
    /// 事業所名
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// 源泉徴収税計算（0: 消費税を含める、1: 消費税を含めない）
    #[serde(rename = "tax_at_source_calc_type")]
    pub tax_at_source_calc_type: i32,
    /// 担当者名 (50文字以内)
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
    /// 従業員数（0: 経営者のみ、1: 2~5人、2: 6~10人、3: 11~20人、4: 21~30人、5: 31~40人、6: 41~100人、7: 100人以上
    #[serde(rename = "head_count")]
    pub head_count: Option<i32>,
    /// 法人番号 (半角数字13桁、法人のみ)
    #[serde(rename = "corporate_number")]
    pub corporate_number: String,
    /// 仕訳番号形式（not_used: 使用しない、digits: 数字（例：5091824）、alnum: 英数字（例：59J0P））
    #[serde(rename = "txn_number_format")]
    pub txn_number_format: TxnNumberFormat,
    /// 決済口座のデフォルト
    #[serde(rename = "default_wallet_account_id", skip_serializing_if = "Option::is_none")]
    pub default_wallet_account_id: Option<i32>,
    /// プライベート資金/役員資金（false: 使用しない、true: 使用する）
    #[serde(rename = "private_settlement")]
    pub private_settlement: bool,
    /// マイナスの表示方法（0: -、 1: △）
    #[serde(rename = "minus_format")]
    pub minus_format: i32,
    /// ユーザーの権限
    #[serde(rename = "role")]
    pub role: Role,
    /// 電話番号１
    #[serde(rename = "phone1")]
    pub phone1: Option<String>,
    /// 電話番号２
    #[serde(rename = "phone2")]
    pub phone2: Option<String>,
    /// FAX
    #[serde(rename = "fax")]
    pub fax: Option<String>,
    /// 郵便番号
    #[serde(rename = "zipcode")]
    pub zipcode: String,
    /// 都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄
    #[serde(rename = "prefecture_code")]
    pub prefecture_code: Option<i32>,
    /// 市区町村・番地
    #[serde(rename = "street_name1")]
    pub street_name1: String,
    /// 建物名・部屋番号など
    #[serde(rename = "street_name2")]
    pub street_name2: String,
    /// 請求書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `carried_forward_standard_classic` - レイアウト３（繰越金額欄あり）/クラシック  * `carried_forward_envelope_classic` - 封筒２（繰越金額欄あり）/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
    #[serde(rename = "invoice_layout")]
    pub invoice_layout: InvoiceLayout,
    /// 金額端数処理方法（0: 切り捨て、1: 切り上げ、2: 四捨五入）
    #[serde(rename = "amount_fraction")]
    pub amount_fraction: i32,
    /// 種別（agriculture_forestry_fisheries_ore: 農林水産業/鉱業、construction: 建設、manufacturing_processing: 製造/加工、it: IT、transportation_logistics: 運輸/物流、retail_wholesale: 小売/卸売、finance_insurance: 金融/保険、real_estate_rental: 不動産/レンタル、profession: 士業/学術/専門技術サービス、design_production: デザイン/制作、food: 飲食、leisure_entertainment: レジャー/娯楽、lifestyle: 生活関連サービス、education: 教育/学習支援、medical_welfare: 医療/福祉、other_services: その他サービス、other: その他）
    #[serde(rename = "industry_class")]
    pub industry_class: IndustryClass,
    /// 業種（agriculture: 農業, forestry: 林業, fishing_industry: 漁業、水産養殖業, mining: 鉱業、採石業、砂利採取業, civil_contractors: 土木工事業, pavement: 舗装工事業, carpenter: とび、大工、左官等の建設工事業, renovation: リフォーム工事業, electrical_plumbing: 電気、管工事等の設備工事業, grocery: 食料品の製造加工業, machinery_manufacturing: 機械器具の製造加工業, printing: 印刷業, other_manufacturing: その他の製造加工業, software_development: 受託：ソフトウェア、アプリ開発業, system_development: 受託：システム開発業, survey_analysis: 受託：調査、分析等の情報処理業, server_management: 受託：サーバー運営管理, website_production: 受託：ウェブサイト制作, online_service_management: オンラインサービス運営業, online_advertising_agency: オンライン広告代理店業, online_advertising_planning_production: オンライン広告企画・制作業, online_media_management: オンラインメディア運営業, portal_site_management: ポータルサイト運営業, other_it_services: その他、IT サービス業, transport_delivery: 輸送業、配送業, delivery: バイク便等の配達業, other_transportation_logistics: その他の運輸業、物流業, other_wholesale: 卸売業：その他, clothing_wholesale_fiber: 卸売業：衣類卸売／繊維, food_wholesale: 卸売業：飲食料品, entrusted_development_wholesale: 卸売業：機械器具, online_shop: 小売業：無店舗　オンラインショップ, fashion_grocery_store: 小売業：店舗あり　ファッション、雑貨, food_store: 小売業：店舗あり　生鮮食品、飲食料品, entrusted_store: 小売業：店舗あり　機械、器具, other_store: 小売業：店舗あり　その他, financial_instruments_exchange: 金融業：金融商品取引, commodity_futures_investment_advisor: 金融業：商品先物取引、商品投資顧問, other_financial: 金融業：その他, brokerage_insurance: 保険業：仲介、代理, other_insurance: 保険業：その他, real_estate_developer: 不動産業：ディベロッパー, real_estate_brokerage: 不動産業：売買、仲介, rent_coin_parking_management: 不動産業：賃貸、コインパーキング、管理, rental_office_co_working_space: 不動産業：レンタルオフィス、コワーキングスペース, rental_lease: レンタル業、リース業, cpa_tax_accountant: 士業：公認会計士事務所、税理士事務所, law_office: 士業：法律事務所, judicial_and_administrative_scrivener: 士業：司法書士事務所／行政書士事務所, labor_consultant: 士業：社会保険労務士事務所, other_profession: 士業：その他, business_consultant: 経営コンサルタント, academic_research_development: 学術・開発研究機関, advertising_agency: 広告代理店, advertising_planning_production: 広告企画／制作, design_development: ソフトウェア、アプリ開発業（受託）, apparel_industry_design: 服飾デザイン業、工業デザイン業, website_design: ウェブサイト制作（受託）, advertising_planning_design: 広告企画／制作業, other_design: その他、デザイン／制作, restaurants_coffee_shops: レストラン、喫茶店等の飲食店業, sale_of_lunch: 弁当の販売業, bread_confectionery_manufacture_sale: パン、菓子等の製造販売業, delivery_catering_mobile_catering: デリバリー業、ケータリング業、移動販売業, hotel_inn: 宿泊業：ホテル、旅館, homestay: 宿泊業：民泊, travel_agency: 旅行代理店業, leisure_sports_facility_management: レジャー、スポーツ等の施設運営業, show_event_management: ショー、イベント等の興行、イベント運営業, barber: ビューティ、ヘルスケア業：床屋、理容室, beauty_salon: ビューティ、ヘルスケア業：美容室, spa_sand_bath_sauna: ビューティ、ヘルスケア業：スパ、砂風呂、サウナ等, este_ail_salon: ビューティ、ヘルスケア業：その他、エステサロン、ネイルサロン等, bridal_planning_introduce_wedding: 冠婚葬祭業：ブライダルプランニング、結婚式場紹介等, memorial_ceremony_funeral: 冠婚葬祭業：メモリアルセレモニー、葬儀等, moving: 引っ越し業, courier_industry: 宅配業, house_maid_cleaning_agency: 家事代行サービス業：無店舗　ハウスメイド、掃除代行等, re_tailoring_clothes: 家事代行サービス業：店舗あり　衣類修理、衣類仕立て直し等, training_institute_management: 研修所等の施設運営業, tutoring_school: 学習塾、進学塾等の教育・学習支援業, music_calligraphy_abacus_classroom: 音楽教室、書道教室、そろばん教室等のの教育・学習支援業, english_school: 英会話スクール等の語学学習支援業, tennis_yoga_judo_school: テニススクール、ヨガ教室、柔道場等のスポーツ指導、支援業, culture_school: その他、カルチャースクール等の教育・学習支援業, seminar_planning_management: セミナー等の企画、運営業, hospital_clinic: 医療業：病院、一般診療所、クリニック等, dental_clinic: 医療業：歯科診療所, other_medical_services: 医療業：その他、医療サービス等, nursery: 福祉業：保育所等、児童向け施設型サービス, nursing_home: 福祉業：老人ホーム等、老人向け施設型サービス, rehabilitation_support_services: 福祉業：療育支援サービス等、障害者等向け施設型サービス, other_welfare: 福祉業：その他、施設型福祉サービス, visit_welfare_service: 福祉業：訪問型福祉サービス, recruitment_temporary_staffing: 人材紹介業、人材派遣業, life_related_recruitment_temporary_staffing: 生活関連サービスの人材紹介業、人材派遣業, car_maintenance_car_repair: 自動車整備業、自動車修理業, machinery_equipment_maintenance_repair: 機械機器類の整備業、修理業, cleaning_maintenance_building_management: 清掃業、メンテナンス業、建物管理業, security: 警備業, other_services: その他のサービス業, npo: NPO, general_incorporated_association: 一般社団法人, general_incorporated_foundation: 一般財団法人, other_association: その他組織)
    #[serde(rename = "industry_code")]
    pub industry_code: IndustryCode,
    /// 仕訳承認フロー（enable: 有効、 disable: 無効）
    #[serde(rename = "workflow_setting")]
    pub workflow_setting: WorkflowSetting,
    /// 取引先コードの利用設定（true: 有効、 false: 無効）
    #[serde(rename = "use_partner_code")]
    pub use_partner_code: bool,
    #[serde(rename = "fiscal_years")]
    pub fiscal_years: Vec<crate::models::FiscalYears>,
    #[serde(rename = "account_items", skip_serializing_if = "Option::is_none")]
    pub account_items: Option<Vec<crate::models::CompanyResponseCompanyAccountItems>>,
    #[serde(rename = "tax_codes", skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<crate::models::CompanyResponseCompanyTaxCodes>>,
    #[serde(rename = "taxes", skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<crate::models::CompanyResponseCompanyTaxes>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::CompanyResponseCompanyItems>>,
    #[serde(rename = "partners", skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<crate::models::CompanyResponseCompanyPartners>>,
    #[serde(rename = "sections", skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<crate::models::CompanyResponseCompanySections>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::CompanyResponseCompanyTags>>,
    #[serde(rename = "walletables", skip_serializing_if = "Option::is_none")]
    pub walletables: Option<Vec<crate::models::CompanyResponseCompanyWalletables>>,
}

impl CompanyResponseCompany {
    pub fn new(id: i32, name: Option<String>, name_kana: Option<String>, display_name: String, tax_at_source_calc_type: i32, contact_name: Option<String>, head_count: Option<i32>, corporate_number: String, txn_number_format: TxnNumberFormat, private_settlement: bool, minus_format: i32, role: Role, phone1: Option<String>, phone2: Option<String>, fax: Option<String>, zipcode: String, prefecture_code: Option<i32>, street_name1: String, street_name2: String, invoice_layout: InvoiceLayout, amount_fraction: i32, industry_class: IndustryClass, industry_code: IndustryCode, workflow_setting: WorkflowSetting, use_partner_code: bool, fiscal_years: Vec<crate::models::FiscalYears>) -> CompanyResponseCompany {
        CompanyResponseCompany {
            id,
            name,
            name_kana,
            display_name,
            tax_at_source_calc_type,
            contact_name,
            head_count,
            corporate_number,
            txn_number_format,
            default_wallet_account_id: None,
            private_settlement,
            minus_format,
            role,
            phone1,
            phone2,
            fax,
            zipcode,
            prefecture_code,
            street_name1,
            street_name2,
            invoice_layout,
            amount_fraction,
            industry_class,
            industry_code,
            workflow_setting,
            use_partner_code,
            fiscal_years,
            account_items: None,
            tax_codes: None,
            taxes: None,
            items: None,
            partners: None,
            sections: None,
            tags: None,
            walletables: None,
        }
    }
}

/// 仕訳番号形式（not_used: 使用しない、digits: 数字（例：5091824）、alnum: 英数字（例：59J0P））
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TxnNumberFormat {
    #[serde(rename = "not_used")]
    NotUsed,
    #[serde(rename = "digits")]
    Digits,
    #[serde(rename = "alnum")]
    Alnum,
}

impl Default for TxnNumberFormat {
    fn default() -> TxnNumberFormat {
        Self::NotUsed
    }
}
/// ユーザーの権限
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "simple_accounting")]
    SimpleAccounting,
    #[serde(rename = "self_only")]
    SelfOnly,
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "workflow")]
    Workflow,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}
/// 請求書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `carried_forward_standard_classic` - レイアウト３（繰越金額欄あり）/クラシック  * `carried_forward_envelope_classic` - 封筒２（繰越金額欄あり）/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceLayout {
    #[serde(rename = "default_classic")]
    DefaultClassic,
    #[serde(rename = "standard_classic")]
    StandardClassic,
    #[serde(rename = "envelope_classic")]
    EnvelopeClassic,
    #[serde(rename = "carried_forward_standard_classic")]
    CarriedForwardStandardClassic,
    #[serde(rename = "carried_forward_envelope_classic")]
    CarriedForwardEnvelopeClassic,
    #[serde(rename = "default_modern")]
    DefaultModern,
    #[serde(rename = "standard_modern")]
    StandardModern,
    #[serde(rename = "envelope_modern")]
    EnvelopeModern,
}

impl Default for InvoiceLayout {
    fn default() -> InvoiceLayout {
        Self::DefaultClassic
    }
}
/// 種別（agriculture_forestry_fisheries_ore: 農林水産業/鉱業、construction: 建設、manufacturing_processing: 製造/加工、it: IT、transportation_logistics: 運輸/物流、retail_wholesale: 小売/卸売、finance_insurance: 金融/保険、real_estate_rental: 不動産/レンタル、profession: 士業/学術/専門技術サービス、design_production: デザイン/制作、food: 飲食、leisure_entertainment: レジャー/娯楽、lifestyle: 生活関連サービス、education: 教育/学習支援、medical_welfare: 医療/福祉、other_services: その他サービス、other: その他）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IndustryClass {
    #[serde(rename = "agriculture_forestry_fisheries_ore")]
    AgricultureForestryFisheriesOre,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "manufacturing_processing")]
    ManufacturingProcessing,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "transportation_logistics")]
    TransportationLogistics,
    #[serde(rename = "retail_wholesale")]
    RetailWholesale,
    #[serde(rename = "finance_insurance")]
    FinanceInsurance,
    #[serde(rename = "real_estate_rental")]
    RealEstateRental,
    #[serde(rename = "profession")]
    Profession,
    #[serde(rename = "design_production")]
    DesignProduction,
    #[serde(rename = "food")]
    Food,
    #[serde(rename = "leisure_entertainment")]
    LeisureEntertainment,
    #[serde(rename = "lifestyle")]
    Lifestyle,
    #[serde(rename = "education")]
    Education,
    #[serde(rename = "medical_welfare")]
    MedicalWelfare,
    #[serde(rename = "other_services")]
    OtherServices,
    #[serde(rename = "other")]
    Other,
}

impl Default for IndustryClass {
    fn default() -> IndustryClass {
        Self::AgricultureForestryFisheriesOre
    }
}
/// 業種（agriculture: 農業, forestry: 林業, fishing_industry: 漁業、水産養殖業, mining: 鉱業、採石業、砂利採取業, civil_contractors: 土木工事業, pavement: 舗装工事業, carpenter: とび、大工、左官等の建設工事業, renovation: リフォーム工事業, electrical_plumbing: 電気、管工事等の設備工事業, grocery: 食料品の製造加工業, machinery_manufacturing: 機械器具の製造加工業, printing: 印刷業, other_manufacturing: その他の製造加工業, software_development: 受託：ソフトウェア、アプリ開発業, system_development: 受託：システム開発業, survey_analysis: 受託：調査、分析等の情報処理業, server_management: 受託：サーバー運営管理, website_production: 受託：ウェブサイト制作, online_service_management: オンラインサービス運営業, online_advertising_agency: オンライン広告代理店業, online_advertising_planning_production: オンライン広告企画・制作業, online_media_management: オンラインメディア運営業, portal_site_management: ポータルサイト運営業, other_it_services: その他、IT サービス業, transport_delivery: 輸送業、配送業, delivery: バイク便等の配達業, other_transportation_logistics: その他の運輸業、物流業, other_wholesale: 卸売業：その他, clothing_wholesale_fiber: 卸売業：衣類卸売／繊維, food_wholesale: 卸売業：飲食料品, entrusted_development_wholesale: 卸売業：機械器具, online_shop: 小売業：無店舗　オンラインショップ, fashion_grocery_store: 小売業：店舗あり　ファッション、雑貨, food_store: 小売業：店舗あり　生鮮食品、飲食料品, entrusted_store: 小売業：店舗あり　機械、器具, other_store: 小売業：店舗あり　その他, financial_instruments_exchange: 金融業：金融商品取引, commodity_futures_investment_advisor: 金融業：商品先物取引、商品投資顧問, other_financial: 金融業：その他, brokerage_insurance: 保険業：仲介、代理, other_insurance: 保険業：その他, real_estate_developer: 不動産業：ディベロッパー, real_estate_brokerage: 不動産業：売買、仲介, rent_coin_parking_management: 不動産業：賃貸、コインパーキング、管理, rental_office_co_working_space: 不動産業：レンタルオフィス、コワーキングスペース, rental_lease: レンタル業、リース業, cpa_tax_accountant: 士業：公認会計士事務所、税理士事務所, law_office: 士業：法律事務所, judicial_and_administrative_scrivener: 士業：司法書士事務所／行政書士事務所, labor_consultant: 士業：社会保険労務士事務所, other_profession: 士業：その他, business_consultant: 経営コンサルタント, academic_research_development: 学術・開発研究機関, advertising_agency: 広告代理店, advertising_planning_production: 広告企画／制作, design_development: ソフトウェア、アプリ開発業（受託）, apparel_industry_design: 服飾デザイン業、工業デザイン業, website_design: ウェブサイト制作（受託）, advertising_planning_design: 広告企画／制作業, other_design: その他、デザイン／制作, restaurants_coffee_shops: レストラン、喫茶店等の飲食店業, sale_of_lunch: 弁当の販売業, bread_confectionery_manufacture_sale: パン、菓子等の製造販売業, delivery_catering_mobile_catering: デリバリー業、ケータリング業、移動販売業, hotel_inn: 宿泊業：ホテル、旅館, homestay: 宿泊業：民泊, travel_agency: 旅行代理店業, leisure_sports_facility_management: レジャー、スポーツ等の施設運営業, show_event_management: ショー、イベント等の興行、イベント運営業, barber: ビューティ、ヘルスケア業：床屋、理容室, beauty_salon: ビューティ、ヘルスケア業：美容室, spa_sand_bath_sauna: ビューティ、ヘルスケア業：スパ、砂風呂、サウナ等, este_ail_salon: ビューティ、ヘルスケア業：その他、エステサロン、ネイルサロン等, bridal_planning_introduce_wedding: 冠婚葬祭業：ブライダルプランニング、結婚式場紹介等, memorial_ceremony_funeral: 冠婚葬祭業：メモリアルセレモニー、葬儀等, moving: 引っ越し業, courier_industry: 宅配業, house_maid_cleaning_agency: 家事代行サービス業：無店舗　ハウスメイド、掃除代行等, re_tailoring_clothes: 家事代行サービス業：店舗あり　衣類修理、衣類仕立て直し等, training_institute_management: 研修所等の施設運営業, tutoring_school: 学習塾、進学塾等の教育・学習支援業, music_calligraphy_abacus_classroom: 音楽教室、書道教室、そろばん教室等のの教育・学習支援業, english_school: 英会話スクール等の語学学習支援業, tennis_yoga_judo_school: テニススクール、ヨガ教室、柔道場等のスポーツ指導、支援業, culture_school: その他、カルチャースクール等の教育・学習支援業, seminar_planning_management: セミナー等の企画、運営業, hospital_clinic: 医療業：病院、一般診療所、クリニック等, dental_clinic: 医療業：歯科診療所, other_medical_services: 医療業：その他、医療サービス等, nursery: 福祉業：保育所等、児童向け施設型サービス, nursing_home: 福祉業：老人ホーム等、老人向け施設型サービス, rehabilitation_support_services: 福祉業：療育支援サービス等、障害者等向け施設型サービス, other_welfare: 福祉業：その他、施設型福祉サービス, visit_welfare_service: 福祉業：訪問型福祉サービス, recruitment_temporary_staffing: 人材紹介業、人材派遣業, life_related_recruitment_temporary_staffing: 生活関連サービスの人材紹介業、人材派遣業, car_maintenance_car_repair: 自動車整備業、自動車修理業, machinery_equipment_maintenance_repair: 機械機器類の整備業、修理業, cleaning_maintenance_building_management: 清掃業、メンテナンス業、建物管理業, security: 警備業, other_services: その他のサービス業, npo: NPO, general_incorporated_association: 一般社団法人, general_incorporated_foundation: 一般財団法人, other_association: その他組織)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IndustryCode {
    #[serde(rename = "agriculture")]
    Agriculture,
    #[serde(rename = "forestry")]
    Forestry,
    #[serde(rename = "fishing_industry")]
    FishingIndustry,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "civil_contractors")]
    CivilContractors,
    #[serde(rename = "pavement")]
    Pavement,
    #[serde(rename = "carpenter")]
    Carpenter,
    #[serde(rename = "renovation")]
    Renovation,
    #[serde(rename = "electrical_plumbing")]
    ElectricalPlumbing,
    #[serde(rename = "grocery")]
    Grocery,
    #[serde(rename = "machinery_manufacturing")]
    MachineryManufacturing,
    #[serde(rename = "printing")]
    Printing,
    #[serde(rename = "other_manufacturing")]
    OtherManufacturing,
    #[serde(rename = "software_development")]
    SoftwareDevelopment,
    #[serde(rename = "system_development")]
    SystemDevelopment,
    #[serde(rename = "survey_analysis")]
    SurveyAnalysis,
    #[serde(rename = "server_management")]
    ServerManagement,
    #[serde(rename = "website_production")]
    WebsiteProduction,
    #[serde(rename = "online_service_management")]
    OnlineServiceManagement,
    #[serde(rename = "online_advertising_agency")]
    OnlineAdvertisingAgency,
    #[serde(rename = "online_advertising_planning_production")]
    OnlineAdvertisingPlanningProduction,
    #[serde(rename = "online_media_management")]
    OnlineMediaManagement,
    #[serde(rename = "portal_site_management")]
    PortalSiteManagement,
    #[serde(rename = "other_it_services")]
    OtherItServices,
    #[serde(rename = "transport_delivery")]
    TransportDelivery,
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "other_transportation_logistics")]
    OtherTransportationLogistics,
    #[serde(rename = "other_wholesale")]
    OtherWholesale,
    #[serde(rename = "clothing_wholesale_fiber")]
    ClothingWholesaleFiber,
    #[serde(rename = "food_wholesale")]
    FoodWholesale,
    #[serde(rename = "entrusted_development_wholesale")]
    EntrustedDevelopmentWholesale,
    #[serde(rename = "online_shop")]
    OnlineShop,
    #[serde(rename = "fashion_grocery_store")]
    FashionGroceryStore,
    #[serde(rename = "food_store")]
    FoodStore,
    #[serde(rename = "entrusted_store")]
    EntrustedStore,
    #[serde(rename = "other_store")]
    OtherStore,
    #[serde(rename = "financial_instruments_exchange")]
    FinancialInstrumentsExchange,
    #[serde(rename = "commodity_futures_investment_advisor")]
    CommodityFuturesInvestmentAdvisor,
    #[serde(rename = "other_financial")]
    OtherFinancial,
    #[serde(rename = "brokerage_insurance")]
    BrokerageInsurance,
    #[serde(rename = "other_insurance")]
    OtherInsurance,
    #[serde(rename = "real_estate_developer")]
    RealEstateDeveloper,
    #[serde(rename = "real_estate_brokerage")]
    RealEstateBrokerage,
    #[serde(rename = "rent_coin_parking_management")]
    RentCoinParkingManagement,
    #[serde(rename = "rental_office_co_working_space")]
    RentalOfficeCoWorkingSpace,
    #[serde(rename = "rental_lease")]
    RentalLease,
    #[serde(rename = "cpa_tax_accountant")]
    CpaTaxAccountant,
    #[serde(rename = "law_office")]
    LawOffice,
    #[serde(rename = "judicial_and_administrative_scrivener")]
    JudicialAndAdministrativeScrivener,
    #[serde(rename = "labor_consultant")]
    LaborConsultant,
    #[serde(rename = "other_profession")]
    OtherProfession,
    #[serde(rename = "business_consultant")]
    BusinessConsultant,
    #[serde(rename = "academic_research_development")]
    AcademicResearchDevelopment,
    #[serde(rename = "advertising_agency")]
    AdvertisingAgency,
    #[serde(rename = "advertising_planning_production")]
    AdvertisingPlanningProduction,
    #[serde(rename = "design_development")]
    DesignDevelopment,
    #[serde(rename = "apparel_industry_design")]
    ApparelIndustryDesign,
    #[serde(rename = "website_design")]
    WebsiteDesign,
    #[serde(rename = "advertising_planning_design")]
    AdvertisingPlanningDesign,
    #[serde(rename = "other_design")]
    OtherDesign,
    #[serde(rename = "restaurants_coffee_shops")]
    RestaurantsCoffeeShops,
    #[serde(rename = "sale_of_lunch")]
    SaleOfLunch,
    #[serde(rename = "bread_confectionery_manufacture_sale")]
    BreadConfectioneryManufactureSale,
    #[serde(rename = "delivery_catering_mobile_catering")]
    DeliveryCateringMobileCatering,
    #[serde(rename = "hotel_inn")]
    HotelInn,
    #[serde(rename = "homestay")]
    Homestay,
    #[serde(rename = "travel_agency")]
    TravelAgency,
    #[serde(rename = "leisure_sports_facility_management")]
    LeisureSportsFacilityManagement,
    #[serde(rename = "show_event_management")]
    ShowEventManagement,
    #[serde(rename = "barber")]
    Barber,
    #[serde(rename = "beauty_salon")]
    BeautySalon,
    #[serde(rename = "spa_sand_bath_sauna")]
    SpaSandBathSauna,
    #[serde(rename = "este_ail_salon")]
    EsteAilSalon,
    #[serde(rename = "bridal_planning_introduce_wedding")]
    BridalPlanningIntroduceWedding,
    #[serde(rename = "memorial_ceremony_funeral")]
    MemorialCeremonyFuneral,
    #[serde(rename = "moving")]
    Moving,
    #[serde(rename = "courier_industry")]
    CourierIndustry,
    #[serde(rename = "house_maid_cleaning_agency")]
    HouseMaidCleaningAgency,
    #[serde(rename = "re_tailoring_clothes")]
    ReTailoringClothes,
    #[serde(rename = "training_institute_management")]
    TrainingInstituteManagement,
    #[serde(rename = "tutoring_school")]
    TutoringSchool,
    #[serde(rename = "music_calligraphy_abacus_classroom")]
    MusicCalligraphyAbacusClassroom,
    #[serde(rename = "english_school")]
    EnglishSchool,
    #[serde(rename = "tennis_yoga_judo_school")]
    TennisYogaJudoSchool,
    #[serde(rename = "culture_school")]
    CultureSchool,
    #[serde(rename = "seminar_planning_management")]
    SeminarPlanningManagement,
    #[serde(rename = "hospital_clinic")]
    HospitalClinic,
    #[serde(rename = "dental_clinic")]
    DentalClinic,
    #[serde(rename = "other_medical_services")]
    OtherMedicalServices,
    #[serde(rename = "nursery")]
    Nursery,
    #[serde(rename = "nursing_home")]
    NursingHome,
    #[serde(rename = "rehabilitation_support_services")]
    RehabilitationSupportServices,
    #[serde(rename = "other_welfare")]
    OtherWelfare,
    #[serde(rename = "visit_welfare_service")]
    VisitWelfareService,
    #[serde(rename = "recruitment_temporary_staffing")]
    RecruitmentTemporaryStaffing,
    #[serde(rename = "life_related_recruitment_temporary_staffing")]
    LifeRelatedRecruitmentTemporaryStaffing,
    #[serde(rename = "car_maintenance_car_repair")]
    CarMaintenanceCarRepair,
    #[serde(rename = "machinery_equipment_maintenance_repair")]
    MachineryEquipmentMaintenanceRepair,
    #[serde(rename = "cleaning_maintenance_building_management")]
    CleaningMaintenanceBuildingManagement,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "other_services")]
    OtherServices,
    #[serde(rename = "npo")]
    Npo,
    #[serde(rename = "general_incorporated_association")]
    GeneralIncorporatedAssociation,
    #[serde(rename = "general_incorporated_foundation")]
    GeneralIncorporatedFoundation,
    #[serde(rename = "other_association")]
    OtherAssociation,
}

impl Default for IndustryCode {
    fn default() -> IndustryCode {
        Self::Agriculture
    }
}
/// 仕訳承認フロー（enable: 有効、 disable: 無効）
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkflowSetting {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "disable")]
    Disable,
}

impl Default for WorkflowSetting {
    fn default() -> WorkflowSetting {
        Self::Enable
    }
}

