use anyhow::Result;
use yxhoo_transit::{DateKind, TransitArgs};

#[tokio::main]
async fn main() -> Result<()> {
    let args = TransitArgs {
        from: "新宿".into(),
        to: "渋谷".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        criteria: None,
        rank: 1,
        options: None,
    };

    let result = yxhoo_transit::transit(&args).await?;
    println!("{:?}", result);
    Ok(())
}
