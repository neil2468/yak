use crate::NodeAddress;
use async_trait::async_trait;

#[async_trait]
pub trait Node: Sync + Send + 'static {
    async fn main_loop(&mut self);

    fn address(&self) -> &NodeAddress;
}
