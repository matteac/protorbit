#[derive(Debug, PartialEq)]
pub enum Version {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2_0,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP0_9 => "HTTP/0.9".to_string(),
            Self::HTTP1_0 => "HTTP/1.0".to_string(),
            Self::HTTP1_1 => "HTTP/1.1".to_string(),
            Self::HTTP2_0 => "HTTP/2.0".to_string(),
        }
    }
}
impl Version {
    pub fn from_string(s: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let s = s.into();
        let s = s.to_uppercase();
        let v = match s.as_str() {
            "HTTP/0.9" => Self::HTTP0_9,
            "HTTP/1.0" => Self::HTTP1_0,
            "HTTP/1.1" => Self::HTTP1_1,
            "HTTP/2.0" => Self::HTTP2_0,
            _ => return Err("Invalid version".into()),
        };
        return Ok(v);
    }
}
