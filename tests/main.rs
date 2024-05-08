mod tuples;

#[tokio::main]
async fn main() -> Result<(), ()> {
    tuples::test().await;
    Ok(())
}
