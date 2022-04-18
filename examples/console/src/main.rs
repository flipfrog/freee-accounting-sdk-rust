extern crate reqwest;
use std::env;
use openapi_sdk::apis::partners_api;
use openapi_sdk::apis::companies_api;
use openapi_sdk::apis::configuration::Configuration;
use openapi_sdk::models::PartnerCreateParams;

#[tokio::main]
async fn main() {
    let oauth_access_token = env::var("RUST_API_EXAMPLE_OAUTH_ACCESS_TOKEN").expect("アクセストークンの取得に失敗しました");

    let config = Configuration {
        base_path: "https://api.freee.co.jp".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: Some(oauth_access_token),
        bearer_access_token: None,
        api_key: None
    };

    // 事業所一覧を取得
    let companies = companies_api::get_companies(&config).await.expect("事業所一覧の取得に失敗しました");
    for company in &companies.companies {
        println!("- company.id: {}, company.display_name: {}",
                 company.id,
                 company.display_name.as_ref().unwrap_or(&"".to_string())
        );
    }

    let company_id: String = env::var("RUST_API_EXAMPLE_COMPANY_ID").expect("事業所IDの取得に失敗しました");
    let company_id: i32 = company_id.trim().parse().expect("事業所IDのパースに失敗しました");

    let params = PartnerCreateParams {
        company_id,
        name: "Rust API SDKテスト".to_string(),
        code: None,
        shortcut1: None,
        shortcut2: None,
        org_code: None,
        country_code: None,
        long_name: None,
        name_kana: None,
        default_title: None,
        phone: None,
        contact_name: None,
        email: None,
        payer_walletable_id: None,
        transfer_fee_handling_side: None,
        address_attributes: None,
        partner_doc_setting_attributes: None,
        partner_bank_account_attributes: None,
        payment_term_attributes: None,
        invoice_payment_term_attributes: None
    };

    // 取引先を作成する
    let new_partner = partners_api::create_partner(&config, params).await.expect("取引先の作成に失敗しました");
    println!("created: partner.id={}, partner.name={}", new_partner.partner.id, new_partner.partner.name);

    // 取引先を取得する
    let partner = partners_api::get_partner(&config, new_partner.partner.id, company_id).await.expect("取引先の取得に失敗しました");
    println!("got: partner.id={}, partner.name={}", partner.partner.id, partner.partner.name);

    // 取引先を削除する
    partners_api::destroy_partner(&config, new_partner.partner.id, company_id).await.expect("取引先の削除に失敗しました");
    println!("destroy: deleted the partner.");
}
