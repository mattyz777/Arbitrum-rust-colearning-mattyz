use alloy::{
    providers::Provider,
    primitives::{U256, utils::format_ether},
};
use anyhow::Result;

const BASE_TRANSFER_GAS_LIMIT: u64 = 21_000;

// 为了作业 task-3 而实现，没有使用最新的EIP-1559
pub async fn get_estimated_gas_fee_eth(provider: &impl Provider) -> Result<String> {
    let gas_price_wei_u128 = provider.get_gas_price().await?;
    let gas_price_wei = U256::from(gas_price_wei_u128);

    let gas_limit = U256::from(BASE_TRANSFER_GAS_LIMIT);

    let gas_fee_wei = gas_price_wei * gas_limit;
    let gas_fee_eth = format_ether(gas_fee_wei);

    Ok(gas_fee_eth.to_string())
}
