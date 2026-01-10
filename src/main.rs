use alloy::{providers::{Provider, ProviderBuilder}, primitives::{Address}};
use anyhow::Result;
use url::Url;

mod level2_balance_query;
use level2_balance_query::balance::get_eth_balance;

async fn get_block_number(provider: &impl Provider) -> Result<u64> {
    let block_number: u64 = provider.get_block_number().await?;
    Ok(block_number)
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url: Url = std::env::var("ARBITRUM_SEPOLIA_RPC")?.parse()?;
    let account_address: Address = std::env::var("ACCOUNT_ADDRESS")?.parse()?;
    println!("{}, {}", rpc_url, account_address);

    let provider = ProviderBuilder::new().connect_http(rpc_url.clone());

    // Task 1
    let block: u64 = get_block_number(&provider).await?;
    println!("Task 1 当前最新区块高度: {}", block);

    // Task 2
    let balance: String = get_eth_balance(&provider, account_address.clone()).await?;
    println!("Task 2 账户余额: {}", balance);

    Ok(())
}