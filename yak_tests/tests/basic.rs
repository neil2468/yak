use anyhow::Result;
use async_trait::async_trait;
use yak_core::{Address, Manager, Worker};

#[test]
fn basic() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(basic_impl())
}

async fn basic_impl() -> Result<()> {
    let mut m = Manager::startup();

    // let addr = Address::try_from("123")?;

    let w = Inlet {};

    m.spawn_worker(Box::new(w)).await;
    m.wait_on_workers().await;
    m.shutdown().await;

    Ok(())
}

struct Inlet;

#[async_trait]
impl Worker for Inlet {
    async fn run(&mut self) {
        println!("Inlet::run()");
    }
}
