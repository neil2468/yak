use crate::Worker;

pub(crate) struct WorkerWrapper {
    core: Box<dyn Worker>,
}

impl WorkerWrapper {
    pub(crate) fn new(core: Box<dyn Worker>) -> Self {
        Self { core }
    }

    pub(crate) async fn run(&mut self) {
        println!("Worker::run()");

        // TODO: Dummy implementataion
        for _ in 0..10 {
            self.core.run().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        println!("Worker::run() done");
    }
}
