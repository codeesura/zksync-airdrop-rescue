pub mod claim;
pub mod common;
pub mod constants;
pub mod transaction_request;
pub mod transfer;

use crate::transactions::constants::{
    CLAIM_CONTRACT_ADDRESS, RECIPIENT_ADDRESS, TOKEN_CONTRACT_ADDRESS,
};
use alloy_primitives::Address;
use alloy_provider::{network::AnyNetwork, Provider};
use alloy_transport::BoxTransport;
use std::{str::FromStr, sync::Arc};

pub struct WalletProcessParams {
    pub provider: Arc<dyn Provider<BoxTransport, AnyNetwork>>,
    pub private_key: String,
    pub chain_id: u64,
    pub claim_contract_address: Address,
    pub token_contract_address: Address,
    pub recipient_address: Address,
    pub index: u64,
    pub amount: u128,
    pub merkle_proof: Vec<String>,
}

impl WalletProcessParams {
    pub fn new(
        provider: Arc<dyn Provider<BoxTransport, AnyNetwork>>,
        private_key: String,
        chain_id: u64,
        index: u64,
        amount: u128,
        merkle_proof: Vec<String>,
    ) -> Self {
        Self {
            provider,
            private_key,
            chain_id,
            claim_contract_address: Address::from_str(CLAIM_CONTRACT_ADDRESS)
                .expect("Invalid claim contract address"),
            token_contract_address: Address::from_str(TOKEN_CONTRACT_ADDRESS)
                .expect("Invalid token contract address"),
            recipient_address: Address::from_str(RECIPIENT_ADDRESS)
                .expect("Invalid recipient address"),
            index,
            amount,
            merkle_proof,
        }
    }
}
