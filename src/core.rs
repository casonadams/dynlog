use crate::level::COLORS;
use crate::level::current_level_filter;
use crate::timestamp::format_iso8601_timestamp;
use log::{Log, Metadata, Record};

pub struct DynLogger;

impl Log for DynLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= current_level_filter()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();
        let timestamp = format_iso8601_timestamp();
        let location = format!(
            "{}:{}",
            record.file().unwrap_or("?"),
            record.line().unwrap_or(0),
        );

        println!(
            "{}{}{} {}{}{} {}{} {}{}{}",
            COLORS.gray,
            timestamp,
            COLORS.reset,
            COLORS.color(level),
            level,
            COLORS.reset,
            COLORS.gray,
            location,
            COLORS.reset,
            record.args(),
            COLORS.reset,
        );
    }

    fn flush(&self) {}
}
