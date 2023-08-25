use std::iter::FromIterator;
use warp::{Rejection, Reply};
use std::env;

pub async fn handle_callback(body: serde_json::Value) -> Result<impl Reply, Rejection> {
  if let Some(events) = body.get("events").and_then(|e| e.as_array()) {
      for event in events {
          if let Some(reply_token) = event.get("replyToken").and_then(|t| t.as_str()) {
              send_reply(reply_token, "こんにちは、Rustからの応答です！").await;
          }
      }
  }
  Ok(warp::reply::json(&body))
}

async fn send_reply(reply_token: &str, message: &str) {
  let url = "https://api.line.me/v2/bot/message/reply";
  let access_token = env::var("YOUR_ACCESS_TOKEN").expect("YOUR_ACCESS_TOKEN not set");

  let headers = reqwest::header::HeaderMap::from_iter(vec![
      (reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token).parse().unwrap()),
  ]);

  let body = serde_json::json!({
      "replyToken": reply_token,
      "messages": [{
          "type": "text",
          "text": message
      }]
  });

  let _ = reqwest::Client::new()
      .post(url)
      .headers(headers)
      .json(&body)
      .send()
      .await;
}
