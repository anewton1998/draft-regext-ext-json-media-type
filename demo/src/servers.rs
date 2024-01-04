use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use headers::HeaderMap;
use tokio::task::JoinSet;

/// This function will send a redirect to the server operating on port 4000.
/// The redirect will be for a URL that will answer with RDAP.
async fn domain_redirect(connect_info: ConnectInfo<SocketAddr>, headers: HeaderMap) -> Redirect {
    tracing::info!(
        "[redirecting server] Serving request from {}",
        connect_info.0
    );
    let rdap_extensions = parse_extensions(headers.get("accept").unwrap().to_str().unwrap());
    rdap_extensions.iter().for_each(|extension| {
        tracing::info!("[redirecting server] client signaled RDAP extension '{extension}'")
    });
    tracing::info!("[redirecting server] redirecting to server on port 4000");
    Redirect::temporary("http://127.0.0.1:4000/ex2/domain/foo.example")
}

/// This function will send an RDAP answer (which is an RDAP error).
async fn domain_answer(
    connect_info: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> impl IntoResponse {
    tracing::info!(
        "[authoritative server] Serving request from {}",
        connect_info.0
    );
    let rdap_extensions = parse_extensions(headers.get("accept").unwrap().to_str().unwrap());
    rdap_extensions.iter().for_each(|extension| {
        tracing::info!("[authoritative server] client signaled RDAP extension '{extension}'")
    });
    tracing::info!("[authoritative server] responding with an unuseful error");
    (
        StatusCode::from_u16(418).unwrap(),
        [("content-type", r#"application/rdap-x;extensions="foo bar""#)],
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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

/// Launches a web server on ports 3000 and 4000.
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let mut join_set = JoinSet::new();
    join_set.spawn(async { server(3000).await });
    join_set.spawn(async { server(4000).await });
    while let Some(join) = join_set.join_next().await {
        tracing::info!("Tasks finished for {join:?}");
    }
}

fn parse_extensions(accept_header: &str) -> Vec<String> {
    accept_header
        .split(',')
        .map(|media_type| media_type.trim())
        .find(|media_type| media_type.starts_with("application/rdap-x"))
        .unwrap_or_default()
        .split(';')
        .find(|parameter| parameter.starts_with("extensions"))
        .unwrap_or_default()
        .trim_start_matches("extensions")
        .trim_start_matches([' ', '=', '"'])
        .trim_end_matches('"')
        .split_terminator(' ')
        .map(String::from)
        .collect::<Vec<String>>()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::parse_extensions;

    #[test]
    fn GIVEN_accept_header_with_no_extensions_WHEN_parse_extensions_THEN_vec_is_empty() {
        // GIVEN
        let header = r#"application/rdap-x;q=0.9"#;

        // WHEN
        let actual = parse_extensions(header);

        // THEN
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn GIVEN_accept_header_with_empty_extensions_WHEN_parse_extensions_THEN_vec_is_empty() {
        // GIVEN
        let header = r#"application/rdap-x;extensions="";q=0.9"#;

        // WHEN
        let actual = parse_extensions(header);

        // THEN
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn GIVEN_accept_header_with_one_extensions_WHEN_parse_extensions_THEN_vec_has_extensions() {
        // GIVEN
        let header = r#"application/rdap-x;extensions="foo";q=0.9"#;

        // WHEN
        let actual = parse_extensions(header);

        // THEN
        assert_eq!(actual.len(), 1);
        assert_eq!(actual.first().unwrap(), "foo");
    }

    #[test]
    fn GIVEN_accept_header_with_two_extensions_WHEN_parse_extensions_THEN_vec_has_extensions() {
        // GIVEN
        let header = r#"application/rdap-x;extensions="foo bar";q=0.9"#;

        // WHEN
        let actual = parse_extensions(header);

        // THEN
        assert_eq!(actual.len(), 2);
        assert_eq!(actual.first().unwrap(), "foo");
        assert_eq!(actual.last().unwrap(), "bar");
    }
}
