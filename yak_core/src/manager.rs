use tokio::task::JoinSet;

use crate::{worker_wrapper::WorkerWrapper, Worker};

pub struct Manager {
    join_set: JoinSet<()>,
}

// TODO: This is not thread-safe? If a Worker is given access to a shared Manager object then it could call the object from another task.

impl Manager {
    pub fn startup() -> Self {
        Self {
            join_set: JoinSet::new(),
        }
    }

    pub async fn spawn_worker(&mut self, core: Box<dyn Worker>) {
        let mut w = WorkerWrapper::new(core);
        self.join_set.spawn(async move { w.run().await });
    }

    pub async fn wait_on_workers(&mut self) {
        while let Some(_) = self.join_set.join_next().await {}
    }

    pub async fn shutdown(&mut self) {
        self.join_set.shutdown().await;
    }
}
