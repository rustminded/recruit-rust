use chrono::{Duration, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct UtcOffsetSet(HashSet<Duration>);

impl UtcOffsetSet {
    pub fn new() -> UtcOffsetSet {
        UtcOffsetSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Duration> {
        self.0.iter()
    }

    pub fn gap(&self, tz: Duration) -> i64 {
        self.iter()
            .map(|x| (x.num_seconds() - tz.num_seconds()).abs())
            .min()
            .expect("todo")
    }
}

impl Eq for UtcOffsetSet {}

impl PartialEq for UtcOffsetSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Extend<Duration> for UtcOffsetSet {
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

impl std::iter::FromIterator<Duration> for UtcOffsetSet {
    fn from_iter<I: IntoIterator<Item = Duration>>(iter: I) -> Self {
        let mut timezone_set = UtcOffsetSet::new();
        timezone_set.extend(iter);
        timezone_set
    }
}

impl From<&'static [Tz]> for UtcOffsetSet {
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
            .collect::<UtcOffsetSet>()
    }
}
