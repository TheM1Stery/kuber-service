use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    msg: String,
}

pub async fn get() -> Json<Response> {
    let resp = Response {
        msg: "Hello!".to_string(),
    };

    Json(resp)
}
