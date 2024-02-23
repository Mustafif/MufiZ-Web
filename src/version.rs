use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version{
    version: String,
    codename: String,
    status: Status,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Status{
    Archived,
    Released,
    InProgress,
}