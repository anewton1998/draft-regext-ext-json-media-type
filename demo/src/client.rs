use reqwest::header;

/// This is a simple client that:
/// * sets the accept headers to application/rdap+json and application/rdap-x+json
/// * follows redirects with the headers on the redirect being automatically set.
///
/// This client also sets a query parameter to show that they do not survive the redirect.
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let url = "http://127.0.0.1:3000/ex1/domain/foo.example?foo&bar";
    tracing::info!("sending reqwest to {url}");
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(
            header::ACCEPT,
            r#"application/rdap+json;q=0.9, application/rdap-x+json;extensions="foo bar";q=1"#,
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
