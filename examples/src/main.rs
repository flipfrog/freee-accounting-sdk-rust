extern crate reqwest;
use openapi_sdk::apis::partners_api::create_partner;
use openapi_sdk::apis::configuration::Configuration;
use openapi_sdk::models::PartnerCreateParams;

#[tokio::main]
async fn main() {
    let config = Configuration {
        base_path: "http://localhost:3000".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: Some("YOUR ACCESS TOKEN HERE".to_string()),
        bearer_access_token: None,
        api_key: None
    };

    let params = PartnerCreateParams {
        company_id: 2,
        name: "rust apiテスト".to_string(),
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
    let p = create_partner(&config, params).await;
    match p {
        Ok(p) => println!("name={}", p.partner.name), // 成功
        Err(e) => { println!("error={:?}", e) } // 失敗
    }
}
