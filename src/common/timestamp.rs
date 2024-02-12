use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use chrono::{prelude::*, DateTime};

#[derive(Debug, PartialEq, PartialOrd)]
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
}

impl FromStr for Timestamp {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = DateTime::parse_from_rfc3339(s)?;
        let dt = dt.with_timezone(&Utc);
        Ok(Timestamp(dt))
    }
}
impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_wb_ts())
    }
}
