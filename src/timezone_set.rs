use chrono::{Duration, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimeZoneSet(HashSet<Duration>);

impl TimeZoneSet {
    pub fn new() -> TimeZoneSet {
        TimeZoneSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Duration> {
        self.0.iter()
    }
}

impl Eq for TimeZoneSet {}

impl PartialEq for TimeZoneSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Extend<Duration> for TimeZoneSet {
    fn extend<I: IntoIterator<Item = Duration>>(&mut self, iter: I) {
        for elem in iter {
            if let Some(v) = self.0.take(&elem) {
                self.0.insert(v);
            } else {
                self.0.insert(elem);
            }
        }
    }
}

impl std::iter::FromIterator<Duration> for TimeZoneSet {
    fn from_iter<I: IntoIterator<Item = Duration>>(iter: I) -> Self {
        let mut timezone_set = TimeZoneSet::new();
        timezone_set.extend(iter);
        timezone_set
    }
}

impl From<&'static [Tz]> for TimeZoneSet {
    fn from(tz_vec: &'static [Tz]) -> Self {
        tz_vec
            .iter()
            .map(|tz| {
                let utc_offset = tz
                    .offset_from_utc_datetime(&Utc::now().naive_utc())
                    .base_utc_offset();
                let dst_offset = tz
                    .offset_from_utc_datetime(&Utc::now().naive_utc())
                    .dst_offset();
                vec![utc_offset, utc_offset + dst_offset]
            })
            .flatten()
            .collect::<TimeZoneSet>()
    }
}
