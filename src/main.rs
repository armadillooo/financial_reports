use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::{net::SocketAddr, sync::RwLock};

use async_session::MemoryStore;
use axum::extract::Query;
use axum::{
    extract::Json,
    headers::{HeaderMap, HeaderValue},
    http, middleware,
    response::{IntoResponse, Redirect},
    routing::get,
    Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::{self, dotenv};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};
use presentation::{
    auth::{OICDClient, OICDData, OICDService, OICDserviceImpl},
    common::{ApiError, JsonBuilder, Utility, UtilityImpl},
    session::{
        session_manage_layer, ItemKey, SessionData, SessionRepositoryImpl, SessionServiceImpl,
    },
    user::{LoginedUserId, USER_ID},
};

type SessionState = Extension<Arc<RwLock<SessionData>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    type Utilities = UtilityImpl;

    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("self_signed_certs");
    tracing::info!("tls config directory = {:?}", base_path);

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service = UserApplicationServiceImpl::new(&user_repository);
    let session_repository = Arc::new(SessionRepositoryImpl::new(MemoryStore::new()));
    let session_service = SessionServiceImpl::new(&session_repository);
    let oicd_client = OICDClient::new(
        "https://accounts.google.com".to_string(),
        "525690818902-l0urmj6r09omclbguobeq6ef1iqr561k.apps.googleusercontent.com".to_string(),
        "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".to_string(),
        "https://127.0.0.1:3000/api/auth/redirect".to_string(),
    )
    .await
    .unwrap();
    let oicd_service = OICDserviceImpl::new(oicd_client);

    let state = UtilityImpl::new(user_service, session_service, oicd_service);

    let app = Router::new()
        .route("/", get(handler))
        .route("/home", get(after_login))
        .route("/api/auth/redirect", get(auth_verify::<Utilities>))
        .route("/api/auth/google", get(auth_google::<Utilities>))
        .layer(middleware::from_fn(session_manage_layer::<Utilities, _>))
        .layer(Extension(state));

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(Extension(session): SessionState) -> impl IntoResponse {
    let key = ItemKey::<i32>::new("counter");
    let counter = session.read().unwrap().item(&key).unwrap_or(0) + 1;
    session.write().unwrap().insert_item(&key, counter).unwrap();

    format!("counter = {}", counter)
}

const OICD_VERIFY_INFO: ItemKey<OICDData> = ItemKey::new("oicd info");

/// googleの認証画面リダイレクト
async fn auth_google<T: Utility>(
    Extension(session): SessionState,
    Extension(utility): Extension<T>,
) -> impl IntoResponse {
    let verify_info = utility.oicd_service().redirect().await;
    let redirect_url = verify_info.auth_url.clone();
    session
        .write()
        .unwrap()
        .insert_item(&OICD_VERIFY_INFO, verify_info)
        .unwrap();

    tracing::info!("oicd info was saved");

    let mut header = HeaderMap::new();
    header.insert(
        http::header::LOCATION,
        HeaderValue::from_str(&redirect_url.to_string()).unwrap(),
    );

    (http::StatusCode::FOUND, header)
}

/// ユーザー認証完了後の検証
async fn auth_verify<T: Utility>(
    Extension(session): SessionState,
    Extension(utility): Extension<T>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let oicd_info = session
        .read()
        .unwrap()
        .item(&OICD_VERIFY_INFO)
        .expect("There is no verify info in the session");

    let code = params.get("code").expect("query param 'code' is not set");
    let state = params.get("state").expect("query param 'state' is not set");
    if let Ok(user) = utility
        .oicd_service()
        .verify(oicd_info, code.to_owned(), state.to_owned())
        .await
    {
        // 不要なデータをSessionから削除
        session.write().unwrap().remove_item(&OICD_VERIFY_INFO);
        session
            .write()
            .unwrap()
            .insert_item(&USER_ID, LoginedUserId::new(user.id))
            .unwrap();
    } else {
        return (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                JsonBuilder::new()
                    .add(ApiError {
                        message: "oicd verify faild",
                    })
                    .build(),
            ),
        )
            .into_response();
    };

    Redirect::to("/home").into_response()
}

async fn after_login(user_id: LoginedUserId) -> impl IntoResponse {
    format!("Hello! ID:{:?}", user_id.to_string())
}
