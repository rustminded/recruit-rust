use chrono::Duration;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimezoneSet(HashSet<Duration>);

impl TimezoneSet {
    pub fn new() -> TimezoneSet {
        TimezoneSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Duration> {
        self.0.iter()
    }
}

impl Eq for TimezoneSet {}

impl PartialEq for TimezoneSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Extend<Duration> for TimezoneSet {
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

impl std::iter::FromIterator<Duration> for TimezoneSet {
    fn from_iter<I: IntoIterator<Item = Duration>>(iter: I) -> Self {
        let mut timezone_set = TimezoneSet::new();
        timezone_set.extend(iter);
        timezone_set
    }
}
