use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;
use url::Url;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url: Url = std::env::var("ARBITRUM_SEPOLIA_RPC")?.parse()?;
    print!("{}", rpc_url);

    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let block_number: u64 = provider.get_block_number().await?;

    println!("当前最新区块高度: {}", block_number);

    Ok(())
}