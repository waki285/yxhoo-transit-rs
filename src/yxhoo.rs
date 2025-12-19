use std::{collections::HashSet, sync::LazyLock};

use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

use crate::{
    args::{SeatPreference, TransitArgs, TransitTicketPreference, WalkingSpeed},
    http::http_client,
    parser::{TransitDto, load_next_data, next_data_to_transit_dto},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum YxhooPlaceKind {
    #[serde(rename(deserialize = "st"))]
    Station,
    #[serde(rename(deserialize = "bu"))]
    BusStation,
    #[serde(rename(deserialize = "la"))]
    Facility,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct YxhooPlace {
    pub suggest: String,
    pub yomi: String,
    pub lat: String,
    pub lon: String,
    pub address: String,
    #[serde(rename(serialize = "type", deserialize = "Id"))]
    pub kind: YxhooPlaceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct YxhooSuggestResponse {
    #[serde(rename(deserialize = "@query"))]
    pub query: String,
    #[serde(rename(deserialize = "@totalResultsAvailable"))]
    pub total_results: u32,
    #[serde(rename(deserialize = "Result"))]
    pub results: Vec<YxhooPlace>,
}

static BASE_URL: LazyLock<String> =
    LazyLock::new(|| "https://transit.yXhoo.co.jp".replace("X", "a"));

pub async fn suggest_places(query: &str) -> anyhow::Result<YxhooSuggestResponse> {
    let client = http_client();
    let response = client
        .get(format!("{}/api/suggest", *BASE_URL))
        .query(&[("value", query)])
        .send()
        .await?;

    let response: YxhooSuggestResponse = response.json().await?;

    Ok(response)
}

#[inline]
fn minute_digits(min: u32) -> (u32, u32) {
    (min / 10, min % 10)
}

pub async fn transit(args: &TransitArgs) -> anyhow::Result<TransitDto> {
    let client = http_client();
    let mut q: Vec<(String, String)> = Vec::new();

    // from / to
    q.push(("from".into(), args.from.clone()));
    q.push(("to".into(), args.to.clone()));

    // date -> y,m,d,hh,m1,m2
    let dt = args.date;
    q.push(("y".into(), dt.year().to_string()));
    q.push(("m".into(), dt.month().to_string()));
    q.push(("d".into(), dt.day().to_string()));
    q.push(("hh".into(), dt.hour().to_string()));

    let (m1, m2) = minute_digits(dt.minute());
    q.push(("m1".into(), m1.to_string()));
    q.push(("m2".into(), m2.to_string()));

    // type (Departure/Arrival/First/Last/NotSpecified)
    q.push(("type".into(), args.date_kind.as_u32().to_string()));

    // criteria
    let s = args.criteria.unwrap_or_default().as_u32();
    q.push(("s".into(), s.to_string()));

    // rank
    q.push(("no".into(), args.rank.to_string()));

    // options
    if let Some(opt) = &args.options {
        if let Some(ticket) = opt.ticket_preference.clone() {
            q.push(("ticket".into(), ticket.as_str().to_string()));
        }
        if let Some(seat) = &opt.seat_preference {
            q.push(("expkind".into(), seat.as_u32().to_string()));
        }
        if let Some(ws) = opt.walking_speed {
            q.push(("ws".into(), ws.as_u32().to_string()));
        }

        let set: HashSet<&'static str> = opt.available_means.iter().map(|m| m.as_str()).collect();

        for key in ["al", "shin", "ex", "hb", "lb", "sr"] {
            let v = if set.contains(key) { "1" } else { "0" };
            q.push((key.into(), v.into()));
        }
    } else {
        q.push((
            "ticket".into(),
            TransitTicketPreference::default().as_str().to_string(),
        ));
        q.push((
            "expkind".into(),
            SeatPreference::default().as_u32().to_string(),
        ));
        q.push(("ws".into(), WalkingSpeed::default().as_u32().to_string()));
        for key in ["al", "shin", "ex", "hb", "lb", "sr"] {
            q.push((key.into(), "1".into()));
        }
    }

    let response = client
        .get(format!("{}/search/print", *BASE_URL))
        .query(&q)
        .send()
        .await?;

    let response: String = response.text().await?;

    let serde_value = load_next_data(&response)?;
    let structured = next_data_to_transit_dto(&serde_value)?;

    Ok(structured)
}

// tests

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[tokio::test]
    async fn test_suggest_places() {
        let query = "新宿";
        let result = suggest_places(query).await.unwrap();
        println!("{:#?}", result);
        assert!(result.total_results > 0);
        assert!(!result.results.is_empty());
    }

    #[tokio::test]
    async fn test_transit() {
        let args = TransitArgs {
            from: "新宿".into(),
            to: "渋谷".into(),
            date: chrono::FixedOffset::east_opt(9 * 3600)
                .unwrap()
                .with_ymd_and_hms(2024, 7, 1, 9, 0, 0)
                .unwrap(),
            date_kind: crate::args::DateKind::DepartureTime,
            criteria: None,
            rank: 1,
            options: None,
        };
        let result = transit(&args).await.unwrap();
        println!("{:#?}", result);
        assert!(result.from.contains("新宿"));
        assert!(result.to.contains("渋谷"));
    }
}
