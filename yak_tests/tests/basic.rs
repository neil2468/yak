use anyhow::Result;
use async_trait::async_trait;
use yak_core::{Address, Manager, Message, Worker};

#[test]
fn basic() -> Result<()> {
    // Tests are not allowed to use #[tokio::main]
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(basic_impl())
}

async fn basic_impl() -> Result<()> {
    let mut m = Manager::startup();

    let (addr1, w1) = (Address::try_from("inlet1")?, Inlet {});
    let (addr2, w2) = (Address::try_from("inlet2")?, Inlet {});
    let (addr3, w3) = (Address::try_from("inlet3")?, Inlet {});

    m.spawn_worker(addr1.clone(), Box::new(w1)).await;
    m.spawn_worker(addr2.clone(), Box::new(w2)).await;
    m.spawn_worker(addr3.clone(), Box::new(w3)).await;

    for i in 0..10 {
        m.send(&addr1, Message::Ping(i + 100)).await?;
        m.send(&addr2, Message::Ping(i + 200)).await?;
        m.send(&addr3, Message::Ping(i + 300)).await?;
    }

    // TODO: Allow user to specify how long (max) to keep the system running.
    // Or have graceful shutdown and forced shutdown options.
    m.shutdown().await;

    Ok(())
}

struct Inlet;

#[async_trait]
impl Worker for Inlet {
    async fn run(&mut self, addr: &Address, msg: &Message) {
        println!("{} Worker::run() {}", addr, msg);
    }
}
