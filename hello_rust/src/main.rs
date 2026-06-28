use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    //Shared Counter
    let counter = Arc::new(Mutex::new(0));
    background_runner(counter.clone());

    sleep(Duration::from_secs(5)).await;

    println!("Fetching data...");

    let data = fetch_data().await;

    println!("Received: {}", data);

    do_stuffs().await;

    let seconds = *counter.lock().await;
    println!("Program finished: ");
    println!("Background task ran for {}seconds",seconds);
}

fn background_runner(counter : Arc<Mutex<i32>>) {
    
    tokio::spawn(async move {
        

        loop {
            sleep(Duration::from_secs(1)).await;
            let mut value = counter.lock().await;
            *value += 1;
        }
    });
}

async fn do_stuffs() {
    let task_one = fetch_data();
    let task_two = fetch_data();

    let (res1, res2) = tokio::join!(task_one, task_two);

    println!("{}", res1);
    println!("{}", res2);
}

async fn fetch_data() -> String {
    sleep(Duration::from_secs(2)).await;

    String::from("Hello from the async world!")
}
