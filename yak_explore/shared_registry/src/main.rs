use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Started");

    let mut j = Vec::new();

    let arc = Arc::new(RwLock::new(Registry::default()));

    {
        let tmp = arc.read().await;
        println!("len = {}", tmp.data.len());
    }

    for i in 1..3 {
        let w = Worker::new(&format!("w{}", i));
        let arc_clone = arc.clone();
        j.push(tokio::spawn(async move { w.main_loop(arc_clone).await }));
    }

    while !j.iter().all(JoinHandle::is_finished) {
        println!("Waiting...");

        {
            let tmp = arc.read().await;
            println!("len = {}", tmp.data.len());
        }

        sleep(Duration::from_millis(50)).await;
    }

    println!("Finished");

    {
        let tmp = arc.read().await;
        println!("len = {}", tmp.data.len());
    }
}
#[derive(Default)]
struct Registry {
    data: BTreeMap<String, u32>,
}

struct Worker {
    name: String,
}

impl Worker {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    async fn main_loop(&self, arc: Arc<RwLock<Registry>>) {
        {
            let mut x = arc.write().await;
            x.data.insert(self.name.clone(), 0);
        }

        for i in 0..10 {
            println!("{} {}", self.name, i);
            let val: u64 = thread_rng().gen_range(10..40) * 10;
            sleep(Duration::from_millis(val)).await;
        }

        {
            let mut x = arc.write().await;
            x.data.remove(&self.name);
        }
    }
}
