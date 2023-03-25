use crate::client::youtube;
use serde_json::{ json };
use reqwest::Method;
use crate::utils::constants::YOUTUBE_SEARCH;

pub async fn find_channel(q: String) -> Result<reqwest::Response, reqwest::Error> {
    let body = json!({
        "params": "EgIQAg%3D%3D",
        "query": q
    });
    
    let response = youtube::youtube_client(body, Method::POST, YOUTUBE_SEARCH).await;
    return response;
}