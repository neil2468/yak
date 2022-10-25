use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinSet,
};

#[tokio::main]
async fn main() {
    println!("Started");

    let mut js = JoinSet::new();
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    js.spawn(async move { foo(rx).await });
    js.spawn(async move { bar(tx).await });

    while let Some(_) = js.join_next().await {}

    println!("Done");
}

async fn foo(mut rx: Receiver<Msg>) {
    let msg = rx.recv().await.unwrap();

    match msg.msg_type {
        MsgType::Control(_) => println!("foo Control {:?}", msg),
        MsgType::User(_) => println!("foo User {:?}", msg),
    }
}

async fn bar(tx: Sender<Msg>) {
    let _ = tx
        .send(Msg {
            msg_type: MsgType::User(String::from("abc")),
            inner: String::from("hello"),
        })
        .await;
}

#[derive(Debug)]
enum MsgType {
    User(String),
    Control(String),
}

#[derive(Debug)]
struct Msg {
    msg_type: MsgType,
    inner: String,
}
