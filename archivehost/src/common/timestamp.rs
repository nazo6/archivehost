use core::fmt;
use std::fmt::{Display, Formatter};

use chrono::{prelude::*, DateTime};
use eyre::OptionExt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct Timestamp(pub DateTime<Utc>);
impl Timestamp {
    /// Convert 14-digit wayback machine timestamp to a `Timestamp`
    pub fn from_wb_ts(s: &str) -> Result<Self, chrono::ParseError> {
        let dt = NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S")?.and_utc();
        let dt = dt.with_timezone(&Utc);
        Ok(Timestamp(dt))
    }

    pub fn to_wb_ts(&self) -> String {
        self.0.format("%Y%m%d%H%M%S").to_string()
    }

    pub fn from_year(s: &str) -> Result<Self, eyre::Report> {
        let last_day = NaiveDate::from_ymd_opt(s.parse()?, 12, 31).ok_or_eyre("Invalid year")?;
        let dt = last_day.and_hms_opt(23, 59, 59).unwrap().and_utc();
        Ok(Timestamp(dt))
    }

    pub fn from_date(s: &str) -> Result<Self, chrono::ParseError> {
        let dt = NaiveDate::parse_from_str(s, "%Y%m%d")?
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();
        Ok(Timestamp(dt))
    }

    /// Parse YYYY / YYYYMMDD / YYYYMMDDhhmmss string
    pub fn from_str(timestamp_str: &str) -> Result<Timestamp, eyre::Error> {
        if let Ok(ts) = Timestamp::from_wb_ts(timestamp_str) {
            Ok(ts)
        } else if let Ok(ts) = Timestamp::from_date(timestamp_str) {
            Ok(ts)
        } else if let Ok(ts) = Timestamp::from_year(timestamp_str) {
            Ok(ts)
        } else {
            Err(eyre::eyre!("Invalid timestamp"))
        }
    }

    pub fn unix_time(&self) -> i64 {
        self.0.timestamp()
    }
    pub fn from_unix_time(ts: i64) -> Result<Self, eyre::Error> {
        match Utc.timestamp_opt(ts, 0) {
            chrono::LocalResult::Single(dt) => Ok(Timestamp(dt)),
            _ => Err(eyre::eyre!("Invalid timestamp")),
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_wb_ts())
    }
}
