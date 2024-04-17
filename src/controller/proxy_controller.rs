use axum::{body::Body, response::Response};
use hyper::{Client, Uri};

pub async fn proxy_request(
    client: &Client<hyper::client::HttpConnector>,
) -> impl axum::response::IntoResponse {
    let uri: Uri = "http://127.0.0.1:9000/list".parse().unwrap();
    let resp = client.get(uri).await.unwrap();
    let res = resp.into_body();
    // res.unwrap()
    todo!()
    // hyper::Response::new(resp.into_body())
}
