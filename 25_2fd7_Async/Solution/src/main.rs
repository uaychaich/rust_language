use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting the program...");

    let future_one = my_async_task(1, 15000);
    let future_two = my_async_task(2, 10000);

    println!("Futures created, but not executed yet.");

    tokio::join!(future_one, future_two);

    println!("Both tasks finished. Program complete.");
}

async fn my_async_task(task_id: u32, sleep_millis: u64) {
    println!("Task {} started (will sleep for {}ms).", task_id, sleep_millis);
    sleep(Duration::from_millis(sleep_millis)).await;

    println!("Task {} finished.", task_id);
}
