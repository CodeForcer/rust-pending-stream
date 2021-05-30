use gumdrop::Options;
use ethers::{
    providers::{Middleware, Provider, Ws, StreamExt},
    types::{TxHash},
};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver};

#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(
        default = "ws://localhost:8546",
        help = "Node Websocket URL"
    )]
    url: String,
}

async fn resolve(mut receiver: Receiver<TxHash>) -> anyhow::Result<()> {
    let opts = Opts::parse_args_default_or_exit();

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;
    let mut count: i32 = 0;

    while let Some(hash) = receiver.recv().await {
        let tx = provider.get_transaction(hash).await.unwrap();
        if tx.is_none() {
            continue;
        }
        println!("{} {:?}", count, hash);
        count = count + 1;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    println!("[pending-stream]");

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;

    let mut watcher = provider.watch_pending_transactions().await?;

    let (sender, receiver) = mpsc::channel(1_000_000);

    tokio::spawn(async move {
        resolve(receiver).await.ok();
    });

    while let Some(hash) = watcher.next().await {
        let sender_internal = sender.clone();

        tokio::spawn(async move {
            sender_internal.send(hash).await.ok();
        });
    }

    Ok(())
}
