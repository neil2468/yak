use tokio::sync::mpsc::Receiver;

use crate::{Address, Message, Worker};

pub(crate) struct WorkerWrapper {
    addr: Address,
    worker: Box<dyn Worker>,
    rx: Receiver<Message>,
}

impl WorkerWrapper {
    pub(crate) fn new(addr: Address, worker: Box<dyn Worker>, rx: Receiver<Message>) -> Self {
        Self { addr, worker, rx }
    }

    pub(crate) async fn run(&mut self) {
        println!("Worker::run()");

        // TODO: Dummy implementataion

        while let Some(msg) = self.rx.recv().await {
            println!("{}: rx'ed {}", self.addr, msg);
            match msg {
                Message::Shutdown => {
                    self.rx.close();
                }
                msg => self.worker.run(&self.addr, &msg).await,
            }
        }

        println!("Worker::run() done");
    }
}
