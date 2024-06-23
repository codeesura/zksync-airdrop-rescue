use super::{
    common::{encode_transfer, get_private_key},
    transaction_request::{create_transaction_request, sign_transaction, TransactionParams},
};
use crate::transactions::constants::TRANSFER_GAZ_LIMIT;
use alloy_primitives::{Address, U256};
use alloy_provider::{network::AnyNetwork, Provider};
use alloy_signer_wallet::LocalWallet;
use alloy_transport::BoxTransport;
use eyre::Report;
use std::sync::Arc;

pub async fn perform_transfer(
    provider: Arc<dyn Provider<BoxTransport, AnyNetwork>>,
    signer: &LocalWallet,
    chain_id: u64,
    token_contract_address: Address,
    recipient_address: Address,
    amount: u128,
) -> Result<(), Report> {
    let sender_address = signer.address();
    let private_key = get_private_key(signer).await?;

    let amount = U256::from(amount);
    let encoded_bytes = encode_transfer(recipient_address, amount);

    let mut nonce = provider.get_transaction_count(sender_address).await.unwrap();
    nonce += 1;

    loop {
        // Gas price * 2
        let gas_price = provider.get_gas_price().await? * 2;

        let mut transfer_tx = create_transaction_request(TransactionParams {
            nonce,
            from: sender_address,
            to: token_contract_address,
            gas_price,
            gas_limit: TRANSFER_GAZ_LIMIT,
            input: encoded_bytes.clone(),
            chain_id,
        });

        let _ = sign_transaction(&mut transfer_tx, &private_key, chain_id);

        if let Some(raw) = transfer_tx.raw {
            match provider.send_raw_transaction(&raw.0).await {
                Ok(pending_tx) => match pending_tx.register().await {
                    Ok(tx_hash) => {
                        println!("Transfer transaction confirmed. Tx Hash: {:?}", tx_hash);
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
