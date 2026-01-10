use anyhow::Result;
use alloy::{
    providers::Provider,
    primitives::{Address, U256, utils::format_ether},
};

pub async fn get_eth_balance(provider: &impl Provider, address: Address) -> Result<String> {
    let balance_wei: U256 = provider.get_balance(address).await?;
    let balance_eth = format_ether(balance_wei);

    Ok(balance_eth)
}
