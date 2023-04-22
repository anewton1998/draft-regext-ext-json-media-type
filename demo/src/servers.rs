use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use headers::HeaderMap;
use reqwest::StatusCode;
use tokio::task::JoinSet;

/// This function will send a redirect to the server operating on port 4000.
/// The redirect will be for a URL that will answer with RDAP.
async fn domain_redirect(connect_info: ConnectInfo<SocketAddr>, headers: HeaderMap) -> Redirect {
    tracing::info!("Serving request from {}", connect_info.0);
    tracing::info!("accept values: {:?}", headers.get("accept").unwrap());
    tracing::info!("redirecting to server on port 4000");
    Redirect::permanent("http://127.0.0.1:4000/ex2/domain/foo.example")
}

/// This function will send an RDAP answer (which is an RDAP error).
async fn domain_answer(
    connect_info: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> impl IntoResponse {
    tracing::info!("Serving request from {}", connect_info.0);
    tracing::info!("accept values: {:?}", headers.get("accept").unwrap());
    tracing::info!("responding with an unuseful error");
    (
        StatusCode::from_u16(418).unwrap(),
        [(
            "content-type",
            r#"application/extrdap;extensions="foo bar""#,
        )],
        r#"{"errorCode":418,"title": "Your Beverage Choice is Not Available"}"#,
    )
}

/// This function starts a web server on the given port.
/// It will accept two paths, /ex1/domain/... which will redirect to
/// /ex2/domain/... which will answer with an RDAP error.
async fn server(port: u32) {
    tracing::info!("starting server on port {port}");
    let app = Router::new()
        .route("/ex1/domain/:domain", get(domain_redirect))
        .route("/ex2/domain/:domain", get(domain_answer));

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

/// Launches a web server on ports 3000 and 4000.
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let mut join_set = JoinSet::new();
    join_set.spawn(async { server(3000).await });
    join_set.spawn(async { server(4000).await });
    while let Some(join) = join_set.join_next().await {
        tracing::info!("Tasks finished for {join:?}");
    }
}
