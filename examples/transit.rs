use anyhow::Result;
use yxhoo_transit::args::{DateKind, TransitArgs};

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

    let result = yxhoo_transit::transit_dto(&args).await?;
    println!("{:?}", result);

    let args = TransitArgs {
        from: "仙台".into(),
        to: "沖縄美ら海水族館".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        criteria: None,
        rank: 1,
        options: None,
    };

    let result = yxhoo_transit::transit_dto(&args).await?;
    println!("{:?}", result);
    Ok(())
}
