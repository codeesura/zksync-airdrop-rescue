use super::common::{convert_address_alloy_to_zksync, encode_general_paymaster_input};
use crate::transactions::constants::PAYMASTER_ADDRESS;
use alloy_primitives::{Address, Bytes};
use rlp::RlpStream;
use std::str::FromStr;
use zksync_crypto_primitives::K256PrivateKey;
use zksync_types::{
    transaction_request::{Eip712Meta, PaymasterParams, TransactionRequest},
    web3::Bytes as zkBytes,
    Address as zkAddress, Eip712Domain, L2ChainId, PackedEthSignature, EIP_712_TX_TYPE,
    U256 as zkU256, U64,
};

pub struct TransactionParams {
    pub nonce: u64,
    pub from: Address,
    pub to: Address,
    pub gas_limit: u64,
    pub gas_price: u128,
    pub input: Bytes,
    pub chain_id: u64,
}

pub fn create_transaction_request(params: TransactionParams) -> TransactionRequest {
    TransactionRequest {
        nonce: zkU256::from(params.nonce),
        from: Some(convert_address_alloy_to_zksync(params.from)),
        to: Some(convert_address_alloy_to_zksync(params.to)),
        gas_price: zkU256::from(params.gas_price),
        max_priority_fee_per_gas: Some(zkU256::from(0)),
        gas: zkU256::from(params.gas_limit),
        input: zkBytes::from(params.input),
        transaction_type: Some(U64::from(EIP_712_TX_TYPE)),
        eip712_meta: Some(Eip712Meta {
            gas_per_pubdata: zkU256::from(50_000), // Fix gas per pubdata
            factory_deps: Some(vec![]),  // no factory
            custom_signature: Some(vec![]), // no custom sign
            paymaster_params: Some(PaymasterParams {
                paymaster: zkAddress::from_str(PAYMASTER_ADDRESS).unwrap(), // General paymaster address
                paymaster_input: encode_general_paymaster_input(), // input for general paymaster
            }),
        }),
        chain_id: Some(params.chain_id),
        ..Default::default()
    }
}

pub fn sign_transaction(
    tx: &mut TransactionRequest,
    private_key: &K256PrivateKey,
    chain_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = PackedEthSignature::typed_data_to_signed_bytes(
        &Eip712Domain::new(L2ChainId::from(chain_id as u32)),
        tx,
    );

    let signature = PackedEthSignature::sign_raw(private_key, &msg)?;

    let mut rlp = RlpStream::new();
    tx.rlp(&mut rlp, Some(&signature))?;
    let mut data = rlp.out().to_vec();
    data.insert(0, EIP_712_TX_TYPE);
    tx.raw = Some(zkBytes(data.clone()));
    tx.v = Some(U64::from(signature.v()));
    tx.r = Some(zkU256::from_big_endian(signature.r()));
    tx.s = Some(zkU256::from_big_endian(signature.s()));

    Ok(())
}
