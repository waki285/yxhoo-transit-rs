use anyhow::Result;
use yxhoo_transit::args::{DateKind, TransitArgs};

#[tokio::main]
async fn main() -> Result<()> {
    let args = TransitArgs {
        from: "新宿".into(),
        to: "渋谷".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        rank: 1,
        ..Default::default()
    };

    let result = yxhoo_transit::transit(&args).await?;
    println!("{:?}", result);

    let args = TransitArgs {
        from: "仙台".into(),
        to: "沖縄美ら海水族館".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        rank: 1,
        ..Default::default()
    };

    let result = yxhoo_transit::transit(&args).await?;
    println!("{:?}", result);
    Ok(())
}
