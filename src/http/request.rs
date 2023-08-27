use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Request {
    method: super::method::Method,
    path: String,
    version: super::version::Version,
    headers: HashMap<String, String>,
    body: String,
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

    fn parse_body(body: &str) -> String {
        let chs = body.chars().collect::<Vec<char>>();
        let mut buffer = String::new();

        for ch in chs {
            match ch {
                '\0' => break,
                _ => buffer.push(ch),
            }
        }
        buffer
    }
    fn parse_head(
        head: &str,
    ) -> Result<
        (
            super::method::Method,
            String,
            super::version::Version,
            HashMap<String, String>,
        ),
        Box<dyn std::error::Error>,
    > {
        let mut lines = head.split("\r\n").collect::<Vec<&str>>();
        let first_line = lines[0].split(" ").collect::<Vec<&str>>();
        lines.remove(0);
        let (method, path, version) = (first_line[0], first_line[1], first_line[2]);

        let method = match super::method::Method::from_string(method) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        let version = match super::version::Version::from_string(version) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mut headers = HashMap::new();
        for line in lines {
            let tmp = line.split(":").collect::<Vec<&str>>();
            let (key, value) = (tmp[0], tmp[1]);
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }

        Ok((method, path.to_string(), version, headers))
    }

    pub fn from_string(r: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let r = r.into();
        let (method, path, version, headers) =
            match Self::parse_head(r.split("\r\n\r\n").collect::<Vec<&str>>()[0]) {
                Ok(h) => h,
                Err(e) => return Err(e),
            };
        let body = Self::parse_body(r.split("\r\n\r\n").collect::<Vec<&str>>()[1]);
        Ok(Self {
            method,
            path,
            headers,
            version,
            body,
        })
    }
}

mod test {
    use std::collections::HashMap;

    use crate::http::{method::Method, version::Version};

    #[test]
    fn test() {
        let raw_req = "GET / HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{}";
        let req = super::Request::from_string(raw_req).unwrap();
        let expected_req = super::Request::new(
            Method::GET,
            "/".to_string(),
            Version::HTTP1_1,
            HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            "{}".to_string(),
        );
        assert_eq!(req, expected_req);
    }
}
