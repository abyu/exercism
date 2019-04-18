use chrono::{DateTime, Duration, Utc};
// Returns a Utc DateTime one billion seconds after start.

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let giga_second: Duration = Duration::seconds(1_000_000_000);
    match start.checked_add_signed(giga_second) {
        None => start,
        Some(x) => x,
    }
}
