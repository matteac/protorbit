#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Version {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2_0,
}

impl TryFrom<String> for Version {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "HTTP/0.9" => Ok(Self::HTTP0_9),
            "HTTP/1.0" => Ok(Self::HTTP1_0),
            "HTTP/1.1" => Ok(Self::HTTP1_1),
            "HTTP/2.0" => Ok(Self::HTTP2_0),
            _ => Err(anyhow::anyhow!("Invalid HTTP Version")),
        }
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        match self {
            Self::HTTP0_9 => "HTTP/0.9".into(),
            Self::HTTP1_0 => "HTTP/1.0".into(),
            Self::HTTP1_1 => "HTTP/1.1".into(),
            Self::HTTP2_0 => "HTTP/2.0".into(),
        }
    }
}
