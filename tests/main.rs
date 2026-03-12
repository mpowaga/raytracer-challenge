mod tuples;
mod canvas;

#[tokio::main]
async fn main() -> Result<(), ()> {
    tuples::test().await;
    canvas::test().await;

    Ok(())
}
