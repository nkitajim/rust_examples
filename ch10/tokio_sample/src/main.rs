use tokio::time::Duration;
use tokio::sync::mpsc;
use futures::future;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn mpcs_sample() {
    // let (tx1, mut rx1) = mpsc::channel(2);
    let (tx, mut rx) = mpsc::channel::<String>(5);

    // 複数の送信タスクを作成
    for i in 0..3 {
        let tx = tx.clone();
        task::spawn(async move {
            let msg = format!("Hello from task {i}");
            tx.send(msg).await.unwrap();
        });
    }

    // rx は最後の1つだけ残っている
    drop(tx); // 全ての tx がクローズされるとループ終了

    // 受信タスク
    while let Some(msg) = rx.recv().await {
        println!("Received: {msg}");
    }
}

#[tokio::main]
async fn mutex_sample() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = task::spawn(async move {
            for _ in 0..100 {
                let mut num = counter.lock().await;
                *num += 1;
            }
            println!("Task {i} finished");
        });
        handles.push(handle);
    }

    future::join_all(handles).await;

    println!("counter = {}", *counter.lock().await);
}

#[tokio::main]
async fn multi_thread() {
    let mut handles = Vec::new();
    let t1 = tokio::task::spawn(async {
        println!("Start t1");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("End t1");
    });
    handles.push(t1);
    let t2 = tokio::task::spawn(async {
        // tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Start t2");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("End t2");
    });
    handles.push(t2);

    // tokio::join!(t1, t2);
    future::join_all(handles).await;
}

fn main() {
    multi_thread();
    mutex_sample();
    mpcs_sample();
}
