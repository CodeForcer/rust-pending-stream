use gumdrop::Options;
use ethers::{
    providers::{Middleware, Provider, Http, StreamExt},
};
use std::ops::Not;
use std::convert::TryFrom;
use std::sync::{Arc};
use std::sync::atomic::{AtomicI32,Ordering};

#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(
        default = "http://localhost:8545",
        help = "Node Websocket URL"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    println!("[pending-stream]");

    let provider = Provider::<Http>::try_from(
        opts.url.as_str()
    )?;
    let provider = Arc::new(provider);

    let mut watcher = provider.watch_pending_transactions().await?;

    let count = Arc::new(AtomicI32::new(0));
    
    while let Some(hash) = watcher.next().await {
        let provider = Arc::clone(&provider);

        let count = Arc::clone(&count);        

        tokio::spawn(async move {
            let tx = provider.get_transaction(hash).await.map_err(|err| {
                println!("aborting: {:?}", err);
                ::std::process::abort()
            }).ok();
            let number = count.fetch_add(1, Ordering::SeqCst);
            if tx.is_none().not() {
                println!("{} {:?}", number, hash);
            }
        });
    }

    Ok(())
}
