use crate::{TZ_MAX, TZ_MIN};
use chrono::{Duration, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct UtcOffsetSet(HashSet<Duration>);

impl UtcOffsetSet {
    pub fn new() -> UtcOffsetSet {
        UtcOffsetSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Duration> {
        self.0.iter()
    }

    pub fn gap(&self, tz: Duration) -> Option<i64> {
        self.iter()
            .map(|x| (x.num_seconds() - tz.num_seconds()).abs())
            .min()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

#[derive(Debug, Clone)]
pub struct OffsetSetRanges {
    pub normal: RangeInclusive<Duration>,
    pub primary_negative_cursed: RangeInclusive<Duration>,
    pub secondary_negative_cursed: RangeInclusive<Duration>,
    pub primary_positive_cursed: RangeInclusive<Duration>,
    pub secondary_positive_cursed: RangeInclusive<Duration>,
}

impl OffsetSetRanges {
    pub fn new(selected_timezone: Duration, tz_range: Duration) -> Self {
        let tz_min = Duration::hours(TZ_MIN);
        let tz_max = Duration::hours(TZ_MAX);

        let normal = (selected_timezone - tz_range)..=(selected_timezone + tz_range);

        let primary_negative_cursed = tz_min..=(selected_timezone + tz_range);
        let secondary_negative_cursed = (tz_max
            - tz_range * 2
            - (*primary_negative_cursed.start() - *primary_negative_cursed.end()))
            ..=tz_max;

        let primary_positive_cursed = (selected_timezone - tz_range)..=tz_max;
        let secondary_positive_cursed = tz_min
            ..=(tz_min + tz_range * 2
                - (*primary_positive_cursed.end() - *primary_positive_cursed.start()));

        Self {
            normal,
            primary_negative_cursed,
            secondary_negative_cursed,
            primary_positive_cursed,
            secondary_positive_cursed,
        }
    }
}
