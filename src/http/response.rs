use std::collections::HashMap;

use super::version::Version;

/// HTTP/1.1 200 OK
/// Content-Type: text/html
///
/// <h1>Hello World!</h1>

pub struct Response {
    version: Version,
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}
