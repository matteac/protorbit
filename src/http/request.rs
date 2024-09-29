use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::http::Method;

use super::Version;

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: super::method::Method,
    pub path: String,
    pub version: super::version::Version,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(
        method: super::method::Method,
        path: String,
        version: super::version::Version,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            method,
            path,
            version,
            headers,
            body,
        }
    }

    fn parse(data: impl Into<String>) -> Result<Self> {
        let data = Into::<String>::into(data);
        let mut lines = data.lines();
        let req_line = match lines.next() {
            Some(line) => line,
            None => return Err(anyhow!("No request line")),
        };

        let (method, rest) = match req_line.split_once(" ") {
            Some(kv) => kv,
            None => return Err(anyhow!("Invalid request line")),
        };
        let method = Method::try_from(Into::<String>::into(method))?;
        let (uri, version) = match rest.split_once(" ") {
            Some(kv) => kv,
            None => return Err(anyhow!("Invalid request line")),
        };
        let version = Version::try_from(Into::<String>::into(version))?;
        let mut headers: HashMap<String, String> = HashMap::new();

        while let Some(header) = lines.next() {
            // \r\n\r\n
            if header == "" {
                break;
            }
            if let Some((k, v)) = header.split_once(':') {
                let _ = headers.insert(k.into(), v.into());
            }
        }

        let mut body = String::new();
        while let Some(line) = lines.next() {
            body.push_str(line);
            body.push('\n'); // newline is important in some cases
        }

        Ok(Self {
            method,
            path: uri.into(),
            version,
            headers,
            body,
        })
    }
}

impl TryFrom<String> for Request {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Request::parse(Into::<String>::into(value))
    }
}

impl Into<String> for Request {
    fn into(self) -> String {
        let mut buffer = String::new();
        let s = format!(
            "{} {} {}\r\n",
            Into::<String>::into(self.method),
            self.path,
            Into::<String>::into(self.version),
        );
        buffer.push_str(s.as_str());
        for (h, v) in self.headers.iter() {
            buffer.push_str(&format!("{}: {}\r\n", h, v));
        }
        buffer.push_str("\r\n");
        buffer.push_str(&self.body);
        buffer
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let raw_req = "GET / HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{}";
        let req = super::Request::try_from(Into::<String>::into(raw_req)).unwrap();
        let expected_req = super::Request::new(
            crate::http::method::Method::GET,
            "/".to_string(),
            crate::http::version::Version::HTTP1_1,
            std::collections::HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )]),
            "{}".to_string(),
        );
        assert_eq!(req, expected_req);
        let request_string: String = req.into();
        assert_eq!(raw_req, request_string);
    }
}
