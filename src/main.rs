use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;
use url::Url;

async fn get_block_number(rpc_url: Url) -> Result<u64> {
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let block_number: u64 = provider.get_block_number().await?;
    Ok(block_number)
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url: Url = std::env::var("ARBITRUM_SEPOLIA_RPC")?.parse()?;
    print!("{}", rpc_url);

    let block = get_block_number(rpc_url.clone()).await?;
    println!("当前最新区块高度: {}", block);

    Ok(())
}