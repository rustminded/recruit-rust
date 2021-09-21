use chrono::{Duration, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub const TZ_MIN: i64 = -12;
pub const TZ_MAX: i64 = 14;

#[derive(Debug, Clone)]
pub struct UtcOffsetRange {
    primary: RangeInclusive<Duration>,
    secondary: Option<RangeInclusive<Duration>>,
    timezone: Duration,
}

impl UtcOffsetRange {
    pub fn new(timezone: Duration, range: i64) -> Self {
        let min = Duration::hours(TZ_MIN);
        let max = Duration::hours(TZ_MAX);
        let main_range = Duration::hours(range);

        let (primary, secondary) = if timezone > max - main_range {
            (
                (timezone - main_range)..=max,
                Some(min..=(min + main_range * 2 - (max - (timezone - main_range)))),
            )
        } else if timezone < min + main_range {
            (
                min..=(timezone + main_range),
                Some((max - main_range * 2 - (min - (timezone + main_range)))..=max),
            )
        } else {
            ((timezone - main_range)..=(timezone + main_range), None)
        };

        Self {
            timezone,
            primary,
            secondary,
        }
    }

    pub fn contains(&self, x: Duration) -> bool {
        if let Some(secondary_range) = self.secondary.clone() {
            self.primary.contains(&x) || secondary_range.contains(&x)
        } else {
            self.primary.contains(&x)
        }
    }

    pub fn start(&self) -> &Duration {
        if self.secondary.is_some() {
            if self.timezone.num_hours().is_positive() {
                self.primary.start()
            } else {
                self.primary.end()
            }
        } else {
            self.primary.start()
        }
    }

    pub fn end(&self) -> &Duration {
        if self.secondary.is_some() {
            if self.timezone.num_hours().is_positive() {
                self.secondary.as_ref().unwrap().end()
            } else {
                self.secondary.as_ref().unwrap().start()
            }
        } else {
            self.primary.end()
        }
    }

    pub fn display(&self) -> String {
        format!(
            "UTC {} to {}",
            self.start().num_hours(),
            self.end().num_hours()
        )
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
