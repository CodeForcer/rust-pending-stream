use gumdrop::Options;
use ethers::{
    providers::{Middleware, Provider, Ws, StreamExt},
};
use std::io::Write;
use std::{sync::Arc};

#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(
        default = "ws://localhost:8546",
        help = "Node Websocket URL"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    println!("[rusty-sandwich]");

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;
    run(provider).await    
}

async fn run<M: Middleware + Clone + 'static>(provider: M) -> anyhow::Result<()> {
    let provider = Arc::new(provider);

    let mut watcher = provider.watch_pending_transactions().await?;
    while watcher.next().await.is_some() {
        let block = provider.get_block_number().await?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        writeln!(lock, "Got block: {}", block.as_u64())?;
    }

    Ok(())
}
