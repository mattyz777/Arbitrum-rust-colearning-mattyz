use alloy::{providers::{Provider, ProviderBuilder}, primitives::{Address}};
use anyhow::Result;
use url::Url;

mod level2_balance_query;
use level2_balance_query::balance::get_eth_balance;

mod task3_gas;
use task3_gas::gas::get_estimated_gas_fee_eth;

mod task4_transfer;
use task4_transfer::transfer::send_eth;

mod task5_sol;
use task5_sol::erc20_query::query_usdc_info;

async fn get_block_number(provider: &impl Provider) -> Result<u64> {
    let block_number: u64 = provider.get_block_number().await?;
    Ok(block_number)
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url: Url = std::env::var("ARBITRUM_SEPOLIA_RPC")?.parse()?;
    let account_address: Address = std::env::var("ACCOUNT_ADDRESS")?.parse()?;
    let to_address: Address = std::env::var("TO_ADDRESS")?.parse()?;
    println!("{}, {}, {}", rpc_url, account_address, to_address);

    let provider = ProviderBuilder::new().connect_http(rpc_url.clone());

    // Task 1
    let block: u64 = get_block_number(&provider).await?;
    println!("Task 1 当前最新区块高度: {}", block);

    // Task 2
    let balance: String = get_eth_balance(&provider, account_address.clone()).await?;
    println!("Task 2 账户余额: {}", balance);

    // Task 3
    let gas = get_estimated_gas_fee_eth(&provider).await?;
    println!("Task 3 预估的Gas Fee(ETH): {}", gas);
    
    // Task 4 
    // - 截图在images/task4 文件夹中
    // - 交易记录可查询，交易哈希 0xd09b1abd4dddd459e7d23c84d031d43b19dd27aed2f7514f49c78064a9a5f0af
    // - https://sepolia.arbiscan.io/tx/0xd09b1abd4dddd459e7d23c84d031d43b19dd27aed2f7514f49c78064a9a5f0af?referrer=grok.com
    // - private_key_hex 已隐藏
    let result4 = send_eth(&provider, "", &to_address, "0.0001").await?;
    println!("Task 4 转账 {}", result4);
    
    // Task 5
    // - 截图在images/task5 文件夹中
    let (name, symbol) = query_usdc_info(&provider, account_address).await?;
    println!("Task 5");
    println!("Name:      {}", name);
    println!("Symbol:    {}", symbol);

    Ok(())
}