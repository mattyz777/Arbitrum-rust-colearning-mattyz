use std::str::FromStr;

use alloy::{
    eips::BlockId,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, utils::parse_ether},
    providers::Provider,
    rpc::types::eth::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use anyhow::Result;

fn is_valid_private_key_format(pk: &str) -> bool {
    let clean_pk = pk.strip_prefix("0x").unwrap_or(pk);
    if clean_pk.len() != 64 {
        return false;
    }
    clean_pk.chars().all(|c| c.is_ascii_hexdigit())
}

pub async fn send_eth(
    provider: &impl Provider,
    private_key_hex: &str,
    to: &Address,
    eth_amount: &str, // 如 "0.01"
) -> Result<String> {
    if !is_valid_private_key_format(private_key_hex) {
        dbg!("进入了私钥校验失败的分支");
        return Err(anyhow::anyhow!("私钥key的格式不正确"));
    }

    let signer = PrivateKeySigner::from_str(private_key_hex)?;
    let wallet = EthereumWallet::from(signer);
    let from = wallet.default_signer().address();

    let nonce = provider.get_transaction_count(from).await?;
    let value = parse_ether(eth_amount)?;
    let chain_id = provider.get_chain_id().await?;

    let latest_block = provider
        .get_block(BlockId::latest())
        .await?
        .ok_or_else(|| anyhow::anyhow!("No latest block"))?;

    let base_fee = latest_block.header.base_fee_per_gas
        .ok_or_else(|| anyhow::anyhow!("No base fee in block"))?;
    let priority_fee: u128 = 2_000_000_000; // 2 gwei tip
    let base_fee_u128: u128 = base_fee.into();

    // Add buffer: base + priority * 1.3 (integer math)
    let max_fee_per_gas = base_fee_u128 + (priority_fee * 13 / 10);

    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(*to)
        .with_value(value)
        .with_nonce(nonce)
        .with_chain_id(chain_id)
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(priority_fee)
        .with_max_fee_per_gas(max_fee_per_gas);

    let envelope = tx.build(&wallet).await?;
    let pending = provider.send_tx_envelope(envelope).await?;
    
    let tx_hash = pending.tx_hash();

    println!("Transaction sent → {:#x}", tx_hash);
    
    Ok(format!("{:#x}", tx_hash))
}