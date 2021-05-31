use gumdrop::Options;
use ethers::{
    providers::{Middleware, Provider, Ws, StreamExt},
};
use std::ops::Not;
use std::sync::{Arc, Mutex};

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

    println!("[pending-stream]");

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;
    let provider = Arc::new(provider);

    let mut watcher = provider.watch_pending_transactions().await?;

    let count = Arc::new(Mutex::new(0));

    while let Some(hash) = watcher.next().await {
        let provider = Arc::clone(&provider);

        let count = Arc::clone(&count);

        tokio::spawn(async move {
            let tx = provider.get_transaction(hash).await.unwrap();
            if tx.is_none().not() {
                println!("{} {:?}", *count.lock().unwrap(), hash);
            }
            *count.lock().unwrap() += 1;
        });
    }

    Ok(())
}
