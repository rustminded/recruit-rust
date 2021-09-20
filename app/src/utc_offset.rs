use chrono::{Duration, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct UtcOffsetBounds {
    pub min: Duration,
    pub max: Duration,
    pub main_range: Duration,
}

impl UtcOffsetBounds {
    pub fn new(range: i64) -> Self {
        Self {
            min: Duration::hours(-12),
            max: Duration::hours(14),
            main_range: Duration::hours(range),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UtcOffsetRanges {
    pub normal: RangeInclusive<Duration>,
    pub primary_negative_cursed: RangeInclusive<Duration>,
    pub secondary_negative_cursed: RangeInclusive<Duration>,
    pub primary_positive_cursed: RangeInclusive<Duration>,
    pub secondary_positive_cursed: RangeInclusive<Duration>,
}

impl UtcOffsetRanges {
    pub fn new(selected_timezone: Duration, bounds: UtcOffsetBounds) -> Self {
        let normal =
            (selected_timezone - bounds.main_range)..=(selected_timezone + bounds.main_range);

        let primary_negative_cursed = bounds.min..=*normal.end();
        let secondary_negative_cursed =
            (bounds.max - bounds.main_range * 2 - (bounds.min - *normal.end()))..=bounds.max;

        let primary_positive_cursed = *normal.start()..=bounds.max;
        let secondary_positive_cursed =
            bounds.min..=(bounds.min + bounds.main_range * 2 - (bounds.max - *normal.start()));

        Self {
            normal,
            primary_negative_cursed,
            secondary_negative_cursed,
            primary_positive_cursed,
            secondary_positive_cursed,
        }
    }
}

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
