use crate::transactions::constants::{CHAIN_ID, PROVIDER_URL};
use alloy_provider::{network::AnyNetwork, Provider, ProviderBuilder};
use alloy_transport::BoxTransport;
use eyre::{Report, Result};
use std::sync::Arc;

pub async fn setup_provider() -> Result<Arc<dyn Provider<BoxTransport, AnyNetwork>>, Report> {
    let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
        .with_chain_id(CHAIN_ID)
        .on_builtin(PROVIDER_URL)
        .await?;
    Ok(Arc::new(provider))
}
