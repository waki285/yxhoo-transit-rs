use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let query = "新宿";
    let suggestions = yxhoo_transit::suggest_places(query).await?;
    println!("{:?}", suggestions);

    let query = "美ら海水族館";
    let suggestions = yxhoo_transit::suggest_places(query).await?;
    println!("{:?}", suggestions);
    Ok(())
}
