#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::env;
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer, Result};
use actix_web::http::header;
use askama::Template;
use r2d2_sqlite::{self, SqliteConnectionManager};

type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use rusqlite::{params};

use openapi_sdk::apis::configuration::Configuration;
use openapi_sdk::apis::companies_api;
use openapi_sdk::apis::users_api;
use openapi_sdk::models::{CompanyIndexResponseCompaniesInner};

struct AppState {
    oauth: BasicClient,
    pool: Pool,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(Index.render().unwrap()))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    first_name: &'a String,
    last_name: &'a String,
    companies: &'a Vec<CompanyIndexResponseCompaniesInner>,
}

// 認可後の画面を表示する
async fn home(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    // DBからアクセストークンを取得する
    let conn = app_state.pool.get()
        .expect("DB接続の取得に失敗しました");
    let mut statement = conn.prepare("SELECT token FROM token WHERE application_id = 'EXAMPLE'").unwrap();
    let rows = statement.query_map(params![], |row| {
        let token: String = row.get(0).unwrap();
        Ok(token)
    })
        .expect("DBからのアクセストークンの取得に失敗しました");
    let token = rows.reduce(|_a, t| t).unwrap().unwrap();

    let config = Configuration {
        base_path: "https://api.freee.co.jp".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: Some(token),
        bearer_access_token: None,
        api_key: None
    };

    // APIで事業所の一覧を取得する
    let companies = companies_api::get_companies(&config).await
        .expect("事業所一覧の取得に失敗しました");

    // APIでユーザー情報を取得する
    let me = users_api::get_users_me(&config, Some(false), Some(false)).await
        .expect("ユーザ情報の取得に失敗しました");
    let last_name = me.user.last_name.unwrap_or(Some("".to_string()));
    let first_name = me.user.first_name.unwrap_or(Some("".to_string()));

    let s = HomeTemplate {
        first_name: &first_name.unwrap(),
        last_name: &last_name.unwrap(),
        companies: &Vec::from(companies.companies),
    }
        .render()
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// ログインlinkがクリックされたら、freeeの認可画面にリダイレクトする
async fn login(app_state: web::Data<AppState>) -> Result<HttpResponse, AWError> {
    let (auth_url, _csrf_token) = app_state.oauth
        .authorize_url(CsrfToken::new_random)
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
    )
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String
}
// freeeで認可されると、この画面にリダイレクトされる
async fn auth_callback(
    params: web::Query<AuthRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AWError> {
    let code = params.code.clone();
    let code = AuthorizationCode::new(code.to_string());

    let conn = app_state.pool.get()
        .expect("DB接続を取得できませんでした");

    // アクセストークンを取得する
    let token_res = web::block(move|| app_state.oauth.exchange_code(code).request(http_client).unwrap()).await;

    let token = token_res.unwrap();
    let access_token = token.access_token().secret();
    let refresh_token = token.refresh_token().unwrap().secret();

    // アクセストークンとリフレッシュトークンをDBに保存する
    conn.execute("INSERT INTO token VALUES ('EXAMPLE', ?1, ?2)
                    ON CONFLICT(application_id) do update set refresh_token = ?1, token = ?2;",
                 params![refresh_token, access_token])
        .expect("アクセストークンの保存に失敗しました");

    // 認可後の画面にリダイレクトする
    Ok(HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/home"))
        .finish()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // connect to SQLite
    let manager = SqliteConnectionManager::file("token.db");
    let pool = Pool::new(manager).unwrap();

    // create token table
    let conn = pool.get()
        .expect("DB接続の取得に失敗しました");
    conn.execute("CREATE TABLE IF NOT EXISTS token (
                   application_id text,
                   refresh_token text,
                   token text,
                   PRIMARY KEY(application_id)
                 )", params![])
        .expect("トークン保存用テーブルの作成に失敗しました");

    // oauth2 client
    let auth_url = AuthUrl::new("https://accounts.secure.freee.co.jp/public_api/authorize".to_string())
        .expect("認可画面のURLが正しくありません");

    let redirect_url = RedirectUrl::new("http://localhost:8080/auth_callback".to_string())
        .expect("認可後のリダイレクト先URLが正しくありません");

    let token_url = TokenUrl::new("https://accounts.secure.freee.co.jp/public_api/token".to_string())
        .expect("トークン取得URLが正しくありません");

    let client_id = env::var("RUST_API_EXAMPLE_CLIENT_ID").expect("CLIENT_IDの取得に失敗しました");
    let client_secret = env::var("RUST_API_EXAMPLE_CLIENT_SECRET").expect("CLIENT_SECRETの取得に失敗しました");

    let client =
        BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            auth_url,
            Some(token_url)
        )
            .set_redirect_uri(redirect_url);

    // webサービスを開始する
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                oauth: client.clone(),
                pool: pool.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/auth_callback").route(web::get().to(auth_callback)))
            .service(web::resource("/home").route(web::get().to(home)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
