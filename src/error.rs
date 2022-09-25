use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum HoliError {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}
