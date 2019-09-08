use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const SEC_PER_GIGASEC: i64 = 1_000_000_000;
    start + Duration::seconds(SEC_PER_GIGASEC)
}
