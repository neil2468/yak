use std::marker::PhantomData;

use async_trait::async_trait;
use tokio::task::JoinHandle;

#[async_trait]
trait Node: Send + Sync {
    async fn run(&mut self);
    fn id(&self) -> String;
}

#[derive(Default)]
struct Mng<N>
where
    N: Node,
{
    _phantom: PhantomData<N>,
}

impl<N> Mng<N>
where
    N: Node + 'static,
{
    async fn spawn_node(&mut self, mut n: N) -> JoinHandle<()> {
        tokio::spawn(async move { n.run().await })
    }
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
    // Because Mng is generic over Node, we need a specific instance for each of concrete type
    let mut m1 = Mng::<MyNode1>::default();
    let mut m2 = Mng::<MyNode2>::default();

    let mut j: Vec<JoinHandle<()>> = Vec::new();
    j.push(m1.spawn_node(MyNode1::default()).await);
    j.push(m2.spawn_node(MyNode2::default()).await);

    while !j.iter().all(|j| j.is_finished()) {}
    println!("Done.");
}
