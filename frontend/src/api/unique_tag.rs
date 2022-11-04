use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::HoliError, request};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UniqueTag {
    pub name: String,
    pub count: usize,
}

pub async fn get_unique_tags() -> Result<Vec<UniqueTag>, HoliError> {
    request(Method::GET, "unique_tags", ()).await
}
