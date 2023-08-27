#[derive(Debug, PartialEq)]
pub enum StatusCode {
    OK,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::OK => "200 OK".to_string(),
            Self::BadRequest => "400 Bad Request".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
            Self::InternalServerError => "500 Internal Server Error".to_string(),
        }
    }
}

impl StatusCode {
    pub fn to_number(&self) -> u16 {
        match self {
            Self::OK => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }
    pub fn from_number(n: u16) -> Result<Self, Box<dyn std::error::Error>> {
        match n {
            200 => Ok(Self::OK),
            400 => Ok(Self::BadRequest),
            404 => Ok(Self::NotFound),
            500 => Ok(Self::InternalServerError),
            _ => Err("Invalid status code".into()),
        }
    }
    pub fn from_string(s: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let s = s.into();
        let s = s.to_uppercase().split(" ").collect::<Vec<&str>>()[0].parse::<u16>();
        let m = match s {
            Ok(s) => s,
            Err(e) => return Err(Box::new(e)),
        };
        let s = StatusCode::from_number(m);
        s
    }
}
