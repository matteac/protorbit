use super::{status::StatusCode, version::Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub version: Version,
    pub status_code: StatusCode,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(
        version: Version,
        status_code: StatusCode,
        status_message: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            version,
            status_code,
            status_message,
            headers,
            body,
        }
    }
    fn parse(data: impl Into<String>) -> anyhow::Result<Self> {
        let data = data.into();
        let mut lines = data.lines();
        let res_line = match lines.next() {
            Some(line) => line,
            None => return Err(anyhow::anyhow!("Invalid response line")),
        };
        let (version, rest) = match res_line.split_once(" ") {
            Some((version, res)) => (Version::try_from(Into::<String>::into(version))?, res),
            None => return Err(anyhow::anyhow!("Invalid response line")),
        };
        let (status_code, status_message) = match rest.split_once(" ") {
            Some((code, msg)) => (
                StatusCode::try_from(Into::<String>::into(code))?,
                Into::<String>::into(msg),
            ),
            None => return Err(anyhow::anyhow!("Invalid response line")),
        };

        let mut headers: HashMap<String, String> = HashMap::new();
        while let Some(header) = lines.next() {
            if header == "" {
                // \r\n\r\n
                break;
            }
            let (k, v) = match header.split_once(":") {
                Some((k, v)) => (
                    Into::<String>::into(k.trim()),
                    Into::<String>::into(v.trim()),
                ),
                None => return Err(anyhow::anyhow!("Invalid header")),
            };
            headers.insert(k, v);
        }
        let mut body = String::new();
        while let Some(line) = lines.next() {
            body.push_str(line);
        }
        Ok(Self {
            version,
            status_code,
            status_message,
            headers,
            body,
        })
    }
}

impl TryFrom<String> for Response {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Response::parse(value)
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let mut buffer = String::new();
        let s = format!(
            "{} {} {}\r\n",
            Into::<String>::into(self.version),
            Into::<String>::into(self.status_code),
            self.status_message
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
        let res = Response::try_from(Into::<String>::into(raw_res)).unwrap();
        let expected_res = Response::new(
            Version::HTTP1_1,
            StatusCode::OK,
            "Ok".into(),
            HashMap::from([("Content-Type".into(), "application/json".into())]),
            "{}".into(),
        );
        assert_eq!(res, expected_res);
        let response_string: String = res.into();
        assert_eq!(raw_res, response_string);
    }
}
