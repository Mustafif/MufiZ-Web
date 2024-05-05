use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;
use toml::from_str;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    signature: String,
    description: String,
    example_usage: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Functions {
    pub functions: Vec<Function>,
}

impl Functions {
    async fn from_file(path: &str) -> Result<Self> {
        let s = read_to_string(path).await?;
        Ok(from_str(&s).unwrap())
    }
    pub async fn collections() -> Result<Self> {
        Self::from_file("stdlib/collections.toml").await
    }
    pub async fn conversion() -> Result<Self> {
        Self::from_file("stdlib/conversion.toml").await
    }
    pub async fn fs() -> Result<Self> {
        Self::from_file("stdlib/fs.toml").await
    }
    pub async fn math() -> Result<Self> {
        Self::from_file("stdlib/math.toml").await
    }
    pub async fn time() -> Result<Self> {
        Self::from_file("stdlib/time.toml").await
    }
    pub async fn net() -> Result<Self> {
        Self::from_file("stdlib/net.toml").await
    }
}
