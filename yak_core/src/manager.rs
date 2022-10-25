use crate::{message_channel, worker_wrapper::WorkerWrapper, Address, Message, Worker};
use std::collections::BTreeMap;
use thiserror::Error;
use tokio::{
    sync::mpsc::{error::SendError, Sender},
    task::JoinSet,
};

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("unknown address")]
    UnknownAddress,
    #[error("send error")]
    SendError(#[from] SendError<Message>),
}

pub struct Manager {
    join_set: JoinSet<()>,
    workers: BTreeMap<Address, Sender<Message>>,
}

// TODO: This is not thread-safe? If a Worker is given access to a shared Manager object then it could call this object from another task.
// Shared state needs to be protected.

impl Manager {
    pub fn startup() -> Self {
        Self {
            join_set: JoinSet::new(),
            workers: BTreeMap::new(),
        }
    }

    pub async fn spawn_worker(&mut self, addr: Address, worker: Box<dyn Worker>) {
        // Create channel
        let (tx, rx) = message_channel();

        // Wrap and spawn worker
        let mut w = WorkerWrapper::new(addr.clone(), worker, rx);
        self.join_set.spawn(async move { w.run().await });

        // Update local data
        self.workers.insert(addr, tx);
    }

    pub async fn shutdown(&mut self) {
        // Message workers
        // TODO: a worker whose channel is full will cause this loop to block
        //       and other workers may not be messaged!
        for a in self.workers.keys() {
            let _ = self.send(a, Message::Shutdown).await;
        }

        // Give workers time to shutdown
        // TODO: This should timeout.
        while self.join_set.join_next().await.is_some() {}

        // Force shutdown any workers still remaining
        self.join_set.shutdown().await;
    }

    pub async fn send(&self, addr: &Address, msg: Message) -> Result<(), ManagerError> {
        let tx = match self.workers.get(addr) {
            Some(tx) => tx,
            None => return Err(ManagerError::UnknownAddress),
        };

        match tx.clone().send(msg).await {
            Ok(()) => Ok(()),
            Err(e) => Err(ManagerError::from(e)),
        }
    }
}
