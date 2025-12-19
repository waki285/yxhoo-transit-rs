use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let query = "新宿";
    let suggestions = yxhoo_transit::suggest_places(query).await?;
    println!("{:?}", suggestions);
    Ok(())
}
