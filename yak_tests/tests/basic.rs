use anyhow::Result;
use tokio::time::{sleep, Duration};
use yak_core::{async_trait::async_trait, Node, NodeAddress, NodeAddressError, NodeManager};

struct MyNode {
    addr: NodeAddress,
}

impl MyNode {
    fn from_addr<T: TryInto<NodeAddress, Error = NodeAddressError>>(
        addr: T,
    ) -> Result<Self, NodeAddressError> {
        let tmp: NodeAddress = addr.try_into()?;
        Ok(Self { addr: tmp })
    }
}

#[async_trait]
impl Node for MyNode {
    fn address(&self) -> &NodeAddress {
        &self.addr
    }

    async fn main_loop(&self) {
        for i in 0..10 {
            println!("{}, addr = {}", i, self.addr);
            sleep(Duration::from_millis(250)).await;
        }
    }
}

#[test]
fn basic() -> Result<()> {
    let mut nm = NodeManager::new();

    let node1 = MyNode::from_addr("node#1")?;
    let node2 = MyNode::from_addr("node#2")?;
    let nodex = MyNode::from_addr("node#2")?;
    nm.start_node(node1)?;
    nm.start_node(node2)?;
    let _ = nm.start_node(nodex);

    nm.block_on_nodes();

    Ok(())
}
