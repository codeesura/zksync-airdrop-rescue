use super::{
    common::{encode_claim, get_private_key},
    transaction_request::{create_transaction_request, sign_transaction, TransactionParams},
};
use crate::transactions::constants::CLAIM_GAZ_LIMIT;
use alloy_primitives::Address;
use alloy_provider::{network::AnyNetwork, Provider};
use alloy_signer_wallet::LocalWallet;
use alloy_transport::BoxTransport;
use eyre::Report;
use std::sync::Arc;

pub async fn perform_claim(
    provider: Arc<dyn Provider<BoxTransport, AnyNetwork>>,
    signer: &LocalWallet,
    chain_id: u64,
    claim_contract_address: Address,
    index: u64,
    amount: u128,
    merkle_proof: Vec<String>,
) -> Result<(), Report> {
    let sender_address = signer.address();
    let private_key = get_private_key(signer).await?;

    let claim_bytes = encode_claim(index, amount, merkle_proof);

    let mut nonce = provider.get_transaction_count(sender_address).await.unwrap();
    loop {
        // Gas price * 2
        let gas_price = provider.get_gas_price().await? * 2;

        let mut claim_tx = create_transaction_request(TransactionParams {
            nonce,
            from: sender_address,
            to: claim_contract_address,
            gas_price,
            gas_limit: CLAIM_GAZ_LIMIT,
            input: claim_bytes.clone(),
            chain_id,
        });

        let _ = sign_transaction(&mut claim_tx, &private_key, chain_id);

        if let Some(raw) = claim_tx.raw {
            match provider.send_raw_transaction(&raw.0).await {
                Ok(pending_tx) => match pending_tx.register().await {
                    Ok(tx_hash) => {
                        println!("Claim transaction confirmed. Tx Hash: {:?}", tx_hash);
                        return Ok(());
                    }
                    Err(_) => {
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Error transaction: {:?}", e);
                    if e.to_string().contains("known transaction")
                        || e.to_string().contains("nonce too low")
                    {
                        nonce += 1;
                    } else if e.to_string().contains("nonce too high") {
                        nonce -= 1;
                    }
                    continue;
                }
            }
        } else {
            eprintln!("Error: Transaction raw data is None.");
        }
    }
}
