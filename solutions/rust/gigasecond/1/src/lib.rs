use time::PrimitiveDateTime as DateTime;

const ONE_GIGASECOND: time::Duration = time::Duration::seconds(1_000_000_000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + ONE_GIGASECOND
}
