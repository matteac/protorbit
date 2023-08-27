use super::{status::StatusCode, version::Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub version: Version,
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(
        version: Version,
        status_code: StatusCode,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            version,
            status_code,
            headers,
            body,
        }
    }
    fn parse_head(
        head: &str,
    ) -> Result<
        (
            super::version::Version,
            super::status::StatusCode,
            HashMap<String, String>,
        ),
        Box<dyn std::error::Error>,
    > {
        let mut lines = head.split("\r\n").collect::<Vec<&str>>();
        let first_line = lines[0].split(" ").collect::<Vec<&str>>();
        lines.remove(0);
        let (version, status, _status_message) = (first_line[0], first_line[1], first_line[2]);

        let version = match super::version::Version::from_string(version) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let status = match super::status::StatusCode::from_string(status) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let mut headers = HashMap::new();
        for line in lines {
            let tmp = line.split(":").collect::<Vec<&str>>();
            let (key, value) = (tmp[0], tmp[1]);
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }

        Ok((version, status, headers))
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
    pub fn from_string(r: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let r = r.into();
        let head = r.split("\r\n\r\n").collect::<Vec<&str>>()[0];
        let body = Self::parse_body(r.split("\r\n\r\n").collect::<Vec<&str>>()[1]);
        let (version, status, headers) = match Self::parse_head(head) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        Ok(Self {
            version,
            status_code: status,
            headers,
            body,
        })
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        let s = format!(
            "{} {}\r\n",
            self.version.to_string(),
            self.status_code.to_string(),
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
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test() {
        let raw_res = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}";
        let res = Response::from_string(raw_res).unwrap();
        let expected_res = Response::new(
            Version::HTTP1_1,
            StatusCode::OK,
            HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            "{}".to_string(),
        );
        assert_eq!(res, expected_res);
        let response_string = res.to_string();
        assert_eq!(raw_res, response_string);
    }
}
