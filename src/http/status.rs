#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum StatusCode {
    OK,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl Into<String> for StatusCode {
    fn into(self) -> String {
        match self {
            Self::OK => "200 OK".to_string(),
            Self::BadRequest => "400 Bad Request".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
            Self::InternalServerError => "500 Internal Server Error".to_string(),
        }
    }
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        match self {
            Self::OK => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(Self::OK),
            400 => Ok(Self::BadRequest),
            404 => Ok(Self::NotFound),
            500 => Ok(Self::InternalServerError),
            _ => Err(anyhow::anyhow!("Invalid Status Code")),
        }
    }
}

impl TryFrom<String> for StatusCode {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "200" => Ok(Self::OK),
            "400" => Ok(Self::BadRequest),
            "404" => Ok(Self::NotFound),
            "500" => Ok(Self::InternalServerError),
            _ => Err(anyhow::anyhow!("Invalid Status Code")),
        }
    }
}
