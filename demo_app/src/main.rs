
use my_macros::{Hello, async_task};

#[derive(Hello)]
struct World;

#[async_task]
fn background_job() {
    println!("Doing async work inside background_job()");
}

#[tokio::main]
async fn main() {
    World::hello();

    background_job();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
