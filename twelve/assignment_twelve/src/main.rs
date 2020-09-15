use futures::executor::block_on;
use futures::executor;
use async_std::task;
use std::time::Duration;
use async_std::future::Future;


fn main() {
    block_on(compute());
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(compute());

    let async_data = executor::block_on(async_fashion());
    println!("\nReturned Data asynchronously: {}", async_data);

}

async fn table_two() {
    for i in 1..11 {
        print!("2 * {} = {}\n",i,2*i);
        task::sleep(Duration::from_secs(1)).await;
    }
}

async fn table_three() {
    for i in 1..11 {
        print!("3 * {} = {}\n",i,3*i);
        task::sleep(Duration::from_secs(1)).await;
    }
}

async fn compute() {

    let two_table = table_two();
    let three_table = table_three();

    futures::join!(two_table, three_table);
}

fn async_fashion() -> impl Future<Output = i32> {
    async{
        42
    }
}