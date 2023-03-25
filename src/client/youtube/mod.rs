use crate::utils::{http, constants::YOUTUBE_KEY};
use reqwest::header::{ HeaderMap, COOKIE, REFERER};
use serde_json::{Value, json, Map};
pub mod channel;
pub mod livechat;

pub async fn youtube_client(body: Value, method: reqwest::Method, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let mut headers = HeaderMap::new();
        headers.insert(COOKIE, "".parse().unwrap());
        headers.insert(REFERER, "https://www.youtube.com/".parse().unwrap());
        headers.insert("x-youtube-client-version", "2.20201209.01.00".parse().unwrap());
        headers.insert("x-youtube-client-name", "1".parse().unwrap());

    let mut default_body: Map<String, Value> = serde_json::from_value(json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20230221.06.00",
                "hl": "en",
                "gl": "US",
            }
        }
    }))
    .unwrap();
    let parse_url = reqwest::Url::parse_with_params(&url, &[("key", YOUTUBE_KEY), ("prettyPrint", "false")]).unwrap();
    if let Some(body_map) = body.as_object() {
        default_body.extend(body_map.clone().into_iter());
    };
    let response = http::client(headers, default_body, method, parse_url).await;
    return response;
}
