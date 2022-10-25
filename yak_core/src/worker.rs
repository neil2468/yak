use async_trait::async_trait;

use crate::{Address, Message};

#[async_trait]
pub trait Worker: Send {
    async fn run(&mut self, addr: &Address, msg: &Message);
}
