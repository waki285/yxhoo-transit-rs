use anyhow::{Result, anyhow};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, TimeZone};
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::Value;

/// Parsed transit search result.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitDto {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::dt_minute_tz::option")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "crate::dt_minute_tz::schema"))]
    pub search_date_time: Option<DateTime<FixedOffset>>,
    /// array but must be one route
    pub routes: Vec<RouteDto>,
}

/// A single route in the search result.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteDto {
    pub rank: u32,
    pub summary: RouteSummaryDto,
    pub segments: Vec<SegmentDto>,
}

/// Summary info for a route.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSummaryDto {
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::dt_minute_tz::option")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "crate::dt_minute_tz::schema"))]
    pub departure_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::dt_minute_tz::option")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "crate::dt_minute_tz::schema"))]
    pub arrival_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price_yen: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_km: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_easy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cheap: Option<bool>,
}

/// A segment within a route (rail, walk, bus, etc.).
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentDto {
    pub mode: String, // "rail" | "walk" | "bus" | "flight" | "ferry" | "unknown"
    pub from: String,
    pub to: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fare_yen: Option<u32>,

    // nullable
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::dt_minute_tz::option")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "crate::dt_minute_tz::schema"))]
    pub departure_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "crate::dt_minute_tz::option")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "crate::dt_minute_tz::schema"))]
    pub arrival_time: Option<DateTime<FixedOffset>>,
}

pub fn load_next_data(input: &str) -> Result<Value> {
    let s = input.trim_start();

    // allow JSON
    if s.starts_with('{') {
        return Ok(serde_json::from_str::<Value>(input)?);
    }

    let doc = Html::parse_document(input);
    let sel = Selector::parse(r#"script#__NEXT_DATA__"#).unwrap();
    let json_text = doc
        .select(&sel)
        .next()
        .ok_or_else(|| anyhow!("__NEXT_DATA__ not found in HTML"))?
        .inner_html();

    Ok(serde_json::from_str::<Value>(&json_text)?)
}

pub fn next_data_to_transit_dto(root: &Value) -> Result<TransitDto> {
    let page_props = &root["props"]["pageProps"];
    let navi = &page_props["naviSearchParam"];

    let from = navi["displayInfo"]["fromName"]
        .as_str()
        .or_else(|| page_props["pageQuery"]["from"].as_str())
        .unwrap_or("")
        .to_string();

    let to = navi["displayInfo"]["toName"]
        .as_str()
        .or_else(|| page_props["pageQuery"]["to"].as_str())
        .unwrap_or("")
        .to_string();

    let search_date_time = build_search_datetime(&page_props["pageQuery"]);
    let base_date = search_date_time.as_ref();

    let features = navi["featureInfoList"]
        .as_array()
        .ok_or_else(|| anyhow!("featureInfoList missing"))?;

    let mut routes = Vec::new();

    for (idx, feature) in features.iter().enumerate() {
        let summary = &feature["summaryInfo"];
        let v = vec![];
        let edges = feature["edgeInfoList"].as_array().unwrap_or(&v);

        let departure_time = summary
            .get("departureTime")
            .and_then(as_nonempty_str)
            .and_then(|s| base_date.and_then(|dt| time_on_date_with_rollover(dt, s, None)));

        let arrival_time = summary
            .get("arrivalTime")
            .and_then(as_nonempty_str)
            .and_then(|s| base_date.and_then(|dt| time_on_date_with_rollover(dt, s, departure_time)));

        let route_summary = RouteSummaryDto {
            departure_time,
            arrival_time,
            duration_minutes: summary
                .get("totalTime")
                .and_then(|v| v.as_str())
                .and_then(parse_ja_duration_minutes),
            transfer_count: summary
                .get("transferCount")
                .and_then(|v| v.as_str())
                .and_then(parse_u32_loose),
            total_price_yen: summary
                .get("totalPrice")
                .and_then(|v| v.as_str())
                .and_then(parse_u32_loose),
            distance_km: summary
                .get("distance")
                .and_then(|v| v.as_str())
                .and_then(parse_distance_km),
            is_fast: summary.get("isFast").and_then(|v| v.as_bool()),
            is_easy: summary.get("isEasy").and_then(|v| v.as_bool()),
            is_cheap: summary.get("isCheap").and_then(|v| v.as_bool()),
        };

        let segments = build_segments_from_edges(edges, base_date);

        routes.push(RouteDto {
            rank: (idx as u32) + 1,
            summary: route_summary,
            segments,
        });
    }

    Ok(TransitDto {
        from,
        to,
        search_date_time,
        routes,
    })
}

fn build_segments_from_edges(
    edges: &[Value],
    base_date: Option<&DateTime<FixedOffset>>,
) -> Vec<SegmentDto> {
    let mut out = Vec::new();

    if edges.len() < 2 {
        return out;
    }

    let mut last_time: Option<DateTime<FixedOffset>> = None;

    for i in 0..(edges.len() - 1) {
        let cur = &edges[i];
        let next = &edges[i + 1];

        let from = cur
            .get("stationName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let to = next
            .get("stationName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let line = cur
            .get("railNameExcludingDestination")
            .and_then(|v| as_nonempty_str(v))
            .or_else(|| cur.get("railName").and_then(|v| as_nonempty_str(v)))
            .map(str::to_string);

        let destination = cur
            .get("destination")
            .and_then(|v| as_nonempty_str(v))
            .map(str::to_string);

        let mode = infer_mode(line.as_deref());

        let duration_minutes = cur
            .get("timeOnBoard")
            .and_then(|v| v.as_str())
            .and_then(parse_u32_loose);

        let fare_yen = cur
            .get("priceInfo")
            .and_then(|p| p.get("price"))
            .and_then(|v| v.as_str())
            .and_then(parse_u32_loose);

        let departure_time = cur
            .get("timeInfo")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|x| x.get("time"))
            .and_then(|t| as_nonempty_str(t))
            .and_then(|s| base_date.and_then(|dt| time_on_date_with_rollover(dt, s, last_time)));
        if let Some(dt) = departure_time {
            last_time = Some(dt);
        }

        let arrival_time = next
            .get("timeInfo")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|x| x.get("time"))
            .and_then(|t| as_nonempty_str(t))
            .and_then(|s| base_date.and_then(|dt| time_on_date_with_rollover(dt, s, last_time)));
        if let Some(dt) = arrival_time {
            last_time = Some(dt);
        }

        out.push(SegmentDto {
            mode,
            from,
            to,
            line,
            destination,
            duration_minutes,
            fare_yen,
            departure_time,
            arrival_time,
        });
    }

    out
}

fn infer_mode(line: Option<&str>) -> String {
    let s = line.unwrap_or("");
    if s.contains("徒歩") {
        "walk".to_string()
    } else if s.contains("空路") || s.contains("フライト") || s.contains("飛行機") {
        "flight".to_string()
    } else if s.contains("バス") || s.contains("連絡バス") || s.contains("高速") {
        "bus".to_string()
    } else if s.contains("フェリー") || s.contains("船") {
        "ferry".to_string()
    } else if s.is_empty() {
        "unknown".to_string()
    } else {
        "rail".to_string()
    }
}

fn as_nonempty_str(v: &Value) -> Option<&str> {
    let s = v.as_str()?.trim();
    if s.is_empty() { None } else { Some(s) }
}

fn parse_u32_loose(s: &str) -> Option<u32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

fn parse_ja_duration_minutes(s: &str) -> Option<u32> {
    // "4分", "1時間2分", "3時間" など
    let hours: u32;
    let mut minutes: u32 = 0;

    if let Some(h_pos) = s.find("時間") {
        let h_str = s[..h_pos]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        hours = h_str.parse().ok()?;
        let rest = &s[h_pos + "時間".len()..];
        if let Some(m_pos) = rest.find("分") {
            let m_str = rest[..m_pos]
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            if !m_str.is_empty() {
                minutes = m_str.parse().ok()?;
            }
        }
        return Some(hours * 60 + minutes);
    }

    if let Some(m_pos) = s.find("分") {
        let m_str = s[..m_pos]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        return m_str.parse().ok();
    }

    None
}

fn parse_distance_km(s: &str) -> Option<f64> {
    let t = s.trim().replace(',', "");
    if let Some(km) = t.strip_suffix("km") {
        return km.trim().parse::<f64>().ok();
    }
    if let Some(m) = t.strip_suffix('m') {
        return m.trim().parse::<f64>().ok().map(|v| v / 1000.0);
    }
    None
}

fn build_search_datetime(page_query: &Value) -> Option<DateTime<FixedOffset>> {
    let y = page_query.get("y")?.as_str()?.parse::<i32>().ok()?;
    let m = page_query.get("m")?.as_str()?.parse::<u32>().ok()?;
    let d = page_query.get("d")?.as_str()?.parse::<u32>().ok()?;
    let hh = page_query.get("hh")?.as_str()?.parse::<u32>().ok()?;

    let m1 = page_query
        .get("m1")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let m2 = page_query
        .get("m2")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    let mm = m1 * 10 + m2;

    let date = NaiveDate::from_ymd_opt(y, m, d)?;
    let time = NaiveTime::from_hms_opt(hh, mm, 0)?;
    let dt = date.and_time(time);
    jst_offset().from_local_datetime(&dt).single()
}

fn jst_offset() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).expect("valid JST offset")
}

fn parse_hhmm(s: &str) -> Option<NaiveTime> {
    let mut it = s.split(':');
    let h = it.next()?.trim().parse::<u32>().ok()?;
    let m = it.next()?.trim().parse::<u32>().ok()?;
    if it.next().is_some() {
        return None;
    }
    NaiveTime::from_hms_opt(h, m, 0)
}

fn time_on_date_with_rollover(
    base: &DateTime<FixedOffset>,
    time_str: &str,
    last: Option<DateTime<FixedOffset>>,
) -> Option<DateTime<FixedOffset>> {
    let time = parse_hhmm(time_str)?;
    let date = if let Some(prev) = last {
        let mut date = prev.date_naive();
        if time < prev.time() {
            date = date.succ_opt()?;
        }
        date
    } else {
        base.date_naive()
    };
    let dt = date.and_time(time);
    base.offset().from_local_datetime(&dt).single()
}
