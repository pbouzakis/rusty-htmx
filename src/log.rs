use std::time::{SystemTime, UNIX_EPOCH};
use http::{Method, Uri, StatusCode};
use serde_with::skip_serializing_none;
use serde_json::json;
use serde::Serialize;
use uuid::Uuid;

pub fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    status_code: StatusCode,
) -> () {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        status_code: status_code.to_string(),
    };

	println!("   ->> log_request: \n{:#?}", json!(log_line));
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String, // (iso8601)

    // -- http request attributes
    req_path: String,
    req_method: String,

    // -- http response attributes
    status_code: String,

    // -- Errors and attributes
    // TODO!
}