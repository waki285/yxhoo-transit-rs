use chrono::{DateTime, FixedOffset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dt_minute_tz;

/// Arguments for place suggestions.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SuggestPlaceArgs {
    /// Free-form query string (station name, facility, etc.).
    pub query: String,
}

/// Ticket preference for route search.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransitTicketPreference {
    /// IC card priority
    IC,
    /// Cash / ticket priority
    #[default]
    Normal,
}

impl TransitTicketPreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::IC => "ic",
        }
    }
}

/// Seat preference for limited express / reserved seat services.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SeatPreference {
    #[default]
    NonReserved = 1,
    Reserved = 2,
    GreenCar = 3,
}

impl SeatPreference {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

/// Walking speed when transferring.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WalkingSpeed {
    Fast = 1,
    Brisk = 2,
    #[default]
    Leisurely = 3,
    Slow = 4,
}

impl WalkingSpeed {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

/// Available means of transportation for route search.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AvailableMeans {
    Airlane,
    Shinkansen,
    /// Limited Express Train
    PaidExpress,
    HighwayBus,
    /// Public bus / Articulated bus
    Bus,
    Ferry,
}

impl AvailableMeans {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Airlane => "al",
            Self::Shinkansen => "shin",
            Self::PaidExpress => "ex",
            Self::HighwayBus => "hb",
            Self::Bus => "lb",
            Self::Ferry => "sr",
        }
    }
}

#[inline]
fn default_available_means() -> Vec<AvailableMeans> {
    vec![
        AvailableMeans::Airlane,
        AvailableMeans::Shinkansen,
        AvailableMeans::PaidExpress,
        AvailableMeans::HighwayBus,
        AvailableMeans::Bus,
        AvailableMeans::Ferry,
    ]
}

/// Optional route search options.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TransitOptions {
    /// IC card priority (IC) or cash/ticket priority (Normal).
    pub ticket_preference: Option<TransitTicketPreference>,
    /// Seat preference: non-reserved, reserved, or green car (if applicable).
    pub seat_preference: Option<SeatPreference>,
    /// Walking speed when transferring (default: Leisurely).
    pub walking_speed: Option<WalkingSpeed>,
    /// Available means of transportation (default: all)
    #[serde(default = "default_available_means")]
    pub available_means: Vec<AvailableMeans>,
}

impl Default for TransitOptions {
    fn default() -> Self {
        Self {
            ticket_preference: Some(TransitTicketPreference::default()),
            seat_preference: Some(SeatPreference::default()),
            walking_speed: Some(WalkingSpeed::default()),
            available_means: default_available_means(),
        }
    }
}

/// Date type for route search.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DateKind {
    DepartureTime = 1,
    LastTrain = 2,
    FirstTrain = 3,
    ArrivalTime = 4,
    NotSpecified = 5,
}

impl DateKind {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

/// Criteria to rank routes.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransitCriteria {
    #[default]
    EarliestArrival = 0,
    LowestCost = 1,
    FewestTransfers = 2,
}

impl TransitCriteria {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}


#[inline]
fn default_rank() -> u32 {
    1
}

/// Arguments for transit route search.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TransitArgs {
    /// Origin name (station / place).
    pub from: String,
    /// Destination name (station / place).
    pub to: String,
    /// Date/time with timezone. Minutes precision.
    #[serde(with = "dt_minute_tz")]
    #[schemars(schema_with = "dt_minute_tz::schema")]
    pub date: DateTime<FixedOffset>,
    #[serde(rename = "dateType")]
    /// Interpretation of `date` (departure/arrival/first/last).
    pub date_kind: DateKind,
    /// Criteria for selecting transfers. By default, sorted by earliest arrival.
    pub criteria: Option<TransitCriteria>,
    /// Returns the ranking number based on the criteria. Defaults to 1.
    #[serde(default = "default_rank")]
    pub rank: u32,
    /// Optional route search options.
    pub options: Option<TransitOptions>,
}
