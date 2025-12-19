# yxhoo-transit

某乗換案内サービスの 非公式 Rust クライアントです。
地点候補の提案と経路検索の関数を提供します。

## 使い方

`Cargo.toml` に追加してください:

```toml
[dependencies]
yxhoo-transit = "0.1.0"
```

## Features

HTTP クライアントは feature で切り替えられます (どちらか一方のみ有効化)。

- `http-reqwest` (デフォルト): HTTP クライアントに [reqwest](https://docs.rs/reqwest/latest/reqwest/) を使います。
- `http-wreq`: HTTP クライアントに [wreq](https://docs.rs/wreq/latest/wreq/) を使います。

## 例

```rust
use yxhoo_transit::{suggest_places, transit, TransitArgs, DateKind};

#[tokio::main]
async fn main() {
    // 地点候補の提案
    let suggestions = suggest_places("新宿").await.unwrap();
    println!("{:?}", suggestions);

    // 経路検索
    let args = TransitArgs {
        from: "新宿".into(),
        to: "渋谷".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        criteria: None,
        rank: 1,
        options: None,
    };
    let result = transit(&args).await.unwrap();
    println!("{:?}", result);
}
```

## ライセンス

Apache-2.0
