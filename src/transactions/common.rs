use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use alloy_signer_wallet::LocalWallet;
use alloy_sol_types::{sol, SolCall};
use eyre::Report;
use std::iter::FromIterator;
use zksync_crypto_primitives::K256PrivateKey;
use zksync_types::H256;

sol! {
    function claim(uint256 _index, uint256 _amount, bytes32[] calldata _merkleProof);
    function transfer(address to, uint256 amount) returns (bool);
    function general(bytes input);
}

pub fn encode_claim(index: u64, amount: u128, merkle_proof: Vec<String>) -> Bytes {
    let proof_tokens: Vec<FixedBytes<32>> = merkle_proof
        .into_iter()
        .map(|p| {
            let bytes = hex::decode(p.strip_prefix("0x").unwrap()).unwrap();
            FixedBytes::<32>::from_slice(&bytes)
        })
        .collect();

    let claim_call = claimCall {
        _index: U256::from(index),
        _amount: U256::from(amount),
        _merkleProof: proof_tokens,
    };
    let encoded_claim = claim_call.abi_encode();
    Bytes::from_iter(encoded_claim.iter().cloned())
}

pub fn encode_transfer(to: Address, amount: U256) -> Bytes {
    let transfer_call = transferCall { to, amount };
    let encoded_transfer = transfer_call.abi_encode();
    Bytes::from_iter(encoded_transfer.iter().cloned())
}

pub fn convert_address_alloy_to_zksync(alloy_address: Address) -> zksync_types::Address {
    let bytes: [u8; 20] = alloy_address.into();
    zksync_types::Address::from(bytes)
}

pub fn encode_general_paymaster_input() -> Vec<u8> {
    let input_data: Vec<u8> = vec![];
    let general_call = generalCall { input: Bytes::from(input_data) };
    general_call.abi_encode()
}

pub async fn get_private_key(signer: &LocalWallet) -> Result<K256PrivateKey, Report> {
    let private_key_bytes: H256 = H256::from_slice(signer.to_bytes().as_slice());
    Ok(K256PrivateKey::from_bytes(private_key_bytes)?)
}
