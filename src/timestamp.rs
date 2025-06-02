use time::OffsetDateTime;
use time::macros::format_description;

pub fn format_iso8601_timestamp() -> String {
    const FORMAT: &[time::format_description::FormatItem<'static>] =
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z");

    OffsetDateTime::now_utc()
        .format(FORMAT)
        .unwrap_or_else(|_| "1970-01-01T00:00:00.000Z".into())
}
