use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello").await?.print().await?;

    Ok(())
}
