use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    TRACE,
    PATCH,
}

impl TryFrom<String> for Method {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(anyhow!("Invalid HTTP Method")),
        }
    }
}

impl Into<String> for Method {
    fn into(self) -> String {
        match self {
            Self::GET => "GET".into(),
            Self::HEAD => "HEAD".into(),
            Self::POST => "POST".into(),
            Self::PUT => "PUT".into(),
            Self::DELETE => "DELETE".into(),
            Self::CONNECT => "CONNECT".into(),
            Self::TRACE => "TRACE".into(),
            Self::PATCH => "PATCH".into(),
        }
    }
}
