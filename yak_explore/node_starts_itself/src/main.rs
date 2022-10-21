use async_trait::async_trait;
use tokio::task::JoinHandle;

#[async_trait]
trait Node: Send + Sync {
    async fn run(&mut self);
    fn id(&self) -> String;
}

struct MyNode1 {
    id: String,
    loop_count: u32,
}

impl Default for MyNode1 {
    fn default() -> Self {
        Self {
            id: String::from("myNode1"),
            loop_count: 0,
        }
    }
}

#[async_trait]
impl Node for MyNode1 {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn run(&mut self) {
        for _ in 0..5 {
            self.loop_count += 1;
            println!("loop {}", self.loop_count);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

struct MyNode2 {
    id: String,
    loop_count: u32,
}

impl Default for MyNode2 {
    fn default() -> Self {
        Self {
            id: String::from("myNode2"),
            loop_count: 0,
        }
    }
}

#[async_trait]
impl Node for MyNode2 {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn run(&mut self) {
        for _ in 0..5 {
            self.loop_count += 1;
            println!("shloop {}", self.loop_count);
            tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let nodes: Vec<Box<dyn Node>> =
        vec![Box::new(MyNode1::default()), Box::new(MyNode2::default())];

    let mut j = Vec::new();
    for mut n in nodes {
        j.push(tokio::spawn(async move { n.run().await }));
    }

    while !j.iter().all(|j| j.is_finished()) {}
    println!("Done.");
}
