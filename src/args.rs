use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::dt_minute_tz;

/// Arguments for place suggestions.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Deserialize)]
pub struct SuggestPlaceArgs {
    /// Free-form query string (station name, facility, etc.).
    pub query: String,
}

/// Ticket preference for route search.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(deny_unknown_fields))]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"), deny_unknown_fields)]
pub struct TransitOptions {
    /// IC card priority (IC) or cash/ticket priority (Normal).
    #[serde(default)]
    pub ticket_preference: TransitTicketPreference,
    /// Seat preference: non-reserved, reserved, or green car (if applicable).
    #[serde(default)]
    pub seat_preference: SeatPreference,
    /// Walking speed when transferring (default: Leisurely).
    #[serde(default)]
    pub walking_speed: WalkingSpeed,
    /// Available means of transportation (default: all)
    #[serde(default = "default_available_means")]
    pub available_means: Vec<AvailableMeans>,
}

impl Default for TransitOptions {
    fn default() -> Self {
        Self {
            ticket_preference: TransitTicketPreference::default(),
            seat_preference: SeatPreference::default(),
            walking_speed: WalkingSpeed::default(),
            available_means: default_available_means(),
        }
    }
}

/// Date type for route search.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

impl Default for DateKind {
    fn default() -> Self {
        DateKind::DepartureTime
    }
}

/// Criteria to rank routes.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(deny_unknown_fields))]
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"), deny_unknown_fields)]
pub struct TransitArgs {
    /// Origin name (station / place).
    pub from: String,
    /// Destination name (station / place).
    pub to: String,
    /// Date/time with timezone. Minutes precision.
    #[serde(with = "dt_minute_tz")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "dt_minute_tz::schema"))]
    pub date: DateTime<FixedOffset>,
    #[serde(rename = "dateType")]
    /// Interpretation of `date` (departure/arrival/first/last).
    pub date_kind: DateKind,
    /// Criteria for selecting transfers. By default, sorted by earliest arrival.
    #[serde(default)]
    pub criteria: TransitCriteria,
    /// Returns the ranking number based on the criteria. Defaults to 1.
    #[serde(default = "default_rank")]
    pub rank: u32,
    /// Optional route search options.
    #[serde(default)]
    pub options: TransitOptions,
}
