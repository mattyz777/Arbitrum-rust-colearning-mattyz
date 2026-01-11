use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use anyhow::Result;

use super::erc20::IERC20;

pub async fn query_usdc_info(
    provider: &impl Provider,
    account: Address,
) -> Result<(String, String)> {
    let addr:Address = std::env::var("SEPOLIA_USDC_ADDRESS")?.parse()?;
    let address = IERC20::new(addr, provider);

    let name = address.name().call().await?;
    let symbol   = address.symbol().call().await?;
    // let decimals = address.decimals().call().await?;
    // let balance  = address.balanceOf(account).call().await?;

    Ok((name, symbol))
}
