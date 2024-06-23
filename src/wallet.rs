use eyre::{Report, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize)]
pub struct WalletConfig {
    pub private_keys: Vec<String>,
}

pub async fn load_wallet_config(file_path: &str) -> Result<WalletConfig, Report> {
    let mut file = File::open(file_path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let config: WalletConfig = serde_json::from_slice(&contents)?;
    Ok(config)
}

pub async fn load_airdrop_data(file_path: &str) -> Result<HashMap<String, Value>, Report> {
    let mut file = File::open(file_path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let data: HashMap<String, Value> = serde_json::from_slice(&contents)?;
    Ok(data)
}
