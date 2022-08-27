use subxt::{
    ext::sp_runtime::{
        generic::Header,
        traits::BlakeTwo256,
    },
    rpc::Subscription,
    OnlineClient,
    PolkadotConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api =
        OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // For non-finalised blocks use `.subscribe_blocks()`
    let mut blocks: Subscription<Header<u32, BlakeTwo256>> =
        api.rpc().subscribe_finalized_blocks().await?;

    while let Some(Ok(block)) = blocks.next().await {
        println!(
            "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
            block.number,
            block.hash(),
            block.parent_hash,
            block.state_root,
            block.extrinsics_root
        );
    }

    Ok(())
}

