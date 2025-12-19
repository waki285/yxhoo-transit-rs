use chrono::{DateTime, FixedOffset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dt_minute_tz;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SuggestPlaceArgs {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransitTicketPreference {
    IC,
    Normal,
}

impl Default for TransitTicketPreference {
    fn default() -> Self {
        Self::Normal
    }
}

impl TransitTicketPreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::IC => "ic",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SeatPreference {
    NonReserved = 1,
    Reserved = 2,
    GreenCar = 3,
}

impl SeatPreference {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Default for SeatPreference {
    fn default() -> Self {
        Self::NonReserved
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WalkingSpeed {
    Fast = 1,
    Brisk = 2,
    Leisurely = 3,
    Slow = 4,
}

impl WalkingSpeed {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Default for WalkingSpeed {
    fn default() -> Self {
        Self::Leisurely
    }
}

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

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TransitOptions {
    /// IC Card Priority (IC) or Cash (Ticket) Priority (Normal)
    pub ticket_preference: Option<TransitTicketPreference>,
    /// Seat preference: Non-reserved, Reserved, Green Car (if applicable)
    pub seat_preference: Option<SeatPreference>,
    /// Walking speed when transferring, default is Leisurely
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransitCriteria {
    EarliestArrival = 0,
    LowestCost = 1,
    FewestTransfers = 2,
}

impl TransitCriteria {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Default for TransitCriteria {
    fn default() -> Self {
        Self::EarliestArrival
    }
}

#[inline]
fn default_rank() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TransitArgs {
    pub from: String,
    pub to: String,
    #[serde(with = "dt_minute_tz")]
    #[schemars(schema_with = "dt_minute_tz::schema")]
    pub date: DateTime<FixedOffset>,
    #[serde(rename = "dateType")]
    pub date_kind: DateKind,
    /// Criteria for selecting transfers. By default, sorted by earliest arrival.
    pub criteria: Option<TransitCriteria>,
    /// Returns the ranking number based on the criteria. Defaults to 1.
    #[serde(default = "default_rank")]
    pub rank: u32,
    pub options: Option<TransitOptions>,
}
