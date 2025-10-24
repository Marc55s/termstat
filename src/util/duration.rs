use std::time::Duration;

const SECS_PER_HOUR: u64 = 3600;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;

pub trait DurationExt {
    fn from_days(days: u64) -> Self;

    fn from_weeks(weeks: u64) -> Self;
}

impl DurationExt for Duration {
    fn from_days(days: u64) -> Self {
        Duration::from_secs(days * SECS_PER_DAY)
    }

    fn from_weeks(weeks: u64) -> Self {
        Self::from_days(weeks * 7)
    }
}
