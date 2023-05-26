use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Period {
    pub period: i64,
    pub start: DateTime<Utc>,
    pub end: String,
}

#[derive(Serialize, Deserialize)]
pub struct Periods {
    pub region: String,
    pub previous: Period,
    pub current: Period,
    pub next: Period,
}

#[derive(Serialize, Deserialize)]
pub struct PeriodList {
    pub periods: Vec<Periods>,
}