#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Method {
    pub fn from_string(s: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let s = s.into();
        let s = s.to_uppercase();
        let m = match s.as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "PATCH" => Self::PATCH,
            _ => return Err("Invalid method".into()),
        };
        return Ok(m);
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::GET => "GET".to_string(),
            Self::POST => "POST".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::DELETE => "DELETE".to_string(),
            Self::PATCH => "PATCH".to_string(),
        }
    }
}
