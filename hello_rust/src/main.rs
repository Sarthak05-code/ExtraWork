#[tokio::main]
async fn main() {

    tokio::spawn(async {
        loop {
            println!("Background Runner");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("Fetching data: ");
    let data = fetch_data().await;

    println!("Received data : {}",data);
    do_stuffs().await;
}

async fn do_stuffs() {
    let task_one = fetch_data();
    let task_two = fetch_data();

    let(res1 ,  res2) = tokio::join!(task_one, task_two);
    println!("{}",res1);
    println!("{}",res2);
}

async  fn fetch_data() -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    String::from("Hello from the async world!")
}