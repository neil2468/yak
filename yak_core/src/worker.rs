use async_trait::async_trait;

#[async_trait]
pub trait Worker: Send {
    async fn run(&mut self);
}
