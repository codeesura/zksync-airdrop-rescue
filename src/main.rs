mod provider;
mod transactions;
mod wallet;

use alloy_signer_wallet::LocalWallet;
use dotenv::dotenv;
use eyre::Result;
use futures::future::join_all;
use provider::setup_provider;
use std::{sync::Arc, time::Instant};
use transactions::{claim::perform_claim, transfer::perform_transfer, WalletProcessParams};
use wallet::{load_airdrop_data, load_wallet_config};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenv().ok();

    let provider = setup_provider().await?;
    let provider = Arc::clone(&provider);

    let config = load_wallet_config("wallets.json").await?;
    let chain_id = provider.get_chain_id().await?;
    let start_time = Instant::now();

    let airdrop_data = load_airdrop_data("airdrop_data.json").await?;

    let tasks: Vec<_> = config
        .private_keys
        .into_iter()
        .map(|private_key| {
            let signer: LocalWallet = private_key.parse().unwrap();
            let signer_address = signer.address().to_string();

            if let Some(user_data_array) = airdrop_data.get(&signer_address) {
                let user_data = &user_data_array[0];
                let index = user_data["merkleIndex"].as_str().unwrap().parse::<u64>().unwrap();
                let amount = user_data["tokenAmount"].as_str().unwrap().parse::<u128>().unwrap();
                let merkle_proof: Vec<String> = user_data["merkleProof"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();

                let provider = Arc::clone(&provider);
                let params = WalletProcessParams::new(
                    provider,
                    private_key,
                    chain_id,
                    index,
                    amount,
                    merkle_proof,
                );
                tokio::spawn(async move { process_wallet(params).await })
            } else {
                tokio::spawn(async { Err(eyre::eyre!("User data not found")) })
            }
        })
        .collect();

    let results = join_all(tasks).await;

    for result in results {
        match result {
            Ok(Ok(())) => println!("Task completed successfully"),
            Ok(Err(e)) => eprintln!("Task failed: {:?}", e),
            Err(e) => eprintln!("Task panicked: {:?}", e),
        }
    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("process_wallet duration: {:?}", duration);

    Ok(())
}

async fn process_wallet(params: WalletProcessParams) -> Result<(), eyre::Report> {
    let WalletProcessParams {
        provider,
        private_key,
        chain_id,
        claim_contract_address,
        token_contract_address,
        recipient_address,
        index,
        amount,
        merkle_proof,
    } = params;

    let signer: LocalWallet = private_key.parse().unwrap();
    let signer = Arc::new(signer);
    let provider = Arc::clone(&provider);
    let merkle_proof = Arc::new(merkle_proof);

    loop {
        let claim_future = {
            let provider = Arc::clone(&provider);
            let signer = Arc::clone(&signer);
            let merkle_proof = Arc::clone(&merkle_proof);
            async move {
                match perform_claim(
                    provider.clone(),
                    &signer,
                    chain_id,
                    claim_contract_address,
                    index,
                    amount,
                    merkle_proof.to_vec(),
                )
                .await
                {
                    Ok(_) => {
                        println!("Claim transaction succeeded.");
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Claim transaction failed: {:?}", e);
                        Err(e)
                    }
                }
            }
        };

        let transfer_future = {
            let provider = Arc::clone(&provider);
            let signer = Arc::clone(&signer);
            async move {
                match perform_transfer(
                    provider.clone(),
                    &signer,
                    chain_id,
                    token_contract_address,
                    recipient_address,
                    amount,
                )
                .await
                {
                    Ok(_) => {
                        println!("Transfer transaction succeeded.");
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Transfer transaction failed: {:?}", e);
                        Err(e)
                    }
                }
            }
        };

        let (claim_result, transfer_result) = tokio::join!(claim_future, transfer_future);

        match (claim_result, transfer_result) {
            (Ok(_), Ok(_)) => return Ok(()),
            (Err(claim_err), Ok(_)) => {
                eprintln!("Retrying claim transaction due to error: {:?}", claim_err);
            }
            (Ok(_), Err(transfer_err)) => {
                eprintln!("Retrying transfer transaction due to error: {:?}", transfer_err);
            }
            (Err(claim_err), Err(transfer_err)) => {
                eprintln!(
                    "Retrying both transactions due to errors: claim - {:?}, transfer - {:?}",
                    claim_err, transfer_err
                );
            }
        }
    }
}
