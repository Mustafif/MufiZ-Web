use serde::{Deserialize, Serialize};
use toml::from_str;
use tokio::fs::read_to_string;
use anyhow::Result;

const COLLECTIONS: &str = "stdlib/collections.toml";
const CONVERSION: &str = "stdlib/conversion.toml";
const FS: &str = "stdlib/fs.toml";
const MATH: &str = "stdlib/math.toml";
const TIME: &str = "stdlib/time.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function{
    signature: String,
    description: String,
    example_usage: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Functions{
    pub functions: Vec<Function>
}

impl Functions{
    async fn from_file(path: &str) -> Result<Self>{
        let s = read_to_string(path).await?;
        Ok(from_str(&s).unwrap())
    }
    pub async fn collections() -> Result<Self>{
        Self::from_file(COLLECTIONS).await
    }
    pub async fn conversion() -> Result<Self>{
        Self::from_file(CONVERSION).await
    }
    pub async fn fs() -> Result<Self>{
        Self::from_file(FS).await
    }
    pub async fn math() -> Result<Self>{
        Self::from_file(MATH).await
    }
    pub async fn time() -> Result<Self>{
        Self::from_file(TIME).await
    }
}