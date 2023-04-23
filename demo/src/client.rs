use reqwest::header;

/// This is a simple client that:
/// * sets the accept headers to application/rdap+json and application/extrdap+json
/// * follows redirects with the headers on the redirect being automatically set.
///
/// This client also sets a query parameter to show that they do not survive the redirect.
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("sending reqwest");
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:3000/ex1/domain/foo.example?foo=bar")
        .header(
            header::ACCEPT,
            r#"application/rdap+json, application/extrdap+json;extensions="foo bar""#,
        )
        .send()
        .await
        .unwrap();
    tracing::info!(
        "returned content type: {:?}",
        res.headers().get(header::CONTENT_TYPE).unwrap()
    );
    tracing::info!("status code is {}", res.status());
    tracing::info!("response is {}", res.text().await.unwrap());
}
