use crate::{Node, NodeAddress};
use std::{collections::BTreeMap, sync::Arc};
use thiserror::Error;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinSet,
};

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NodeManagerError {
    #[error("node's address is already in use")]
    AddressInUse,
}

/// Manages all the nodes running in a single async runtime.
pub struct NodeManager {
    nodes: BTreeMap<NodeAddress, Arc<dyn Node>>,
    runtime: Runtime,
    join_set: JoinSet<()>,
}

impl NodeManager {
    pub fn new() -> Self {
        println!("NodeManager::new()");
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap(); // TODO can panic
        Self {
            nodes: BTreeMap::new(),
            runtime,
            join_set: JoinSet::new(),
        }
    }

    pub fn start_node(&mut self, node: impl Node) -> Result<(), NodeManagerError> {
        let addr = node.address().clone();
        if self.nodes.contains_key(&addr) {
            return Err(NodeManagerError::AddressInUse);
        }

        let arc = Arc::new(node);
        let arc_clone = arc.clone();

        self.join_set.spawn_on(
            async move { arc_clone.main_loop().await },
            self.runtime.handle(),
        );

        self.nodes.insert(addr, arc);

        Ok(())
    }

    pub fn block_on_nodes(&mut self) {
        self.runtime
            .block_on(async { while self.join_set.join_next().await.is_some() {} });
    }
}

impl Drop for NodeManager {
    fn drop(&mut self) {
        println!("NodeManager::drop()");
    }
}
