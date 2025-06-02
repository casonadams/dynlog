use log::{Level, LevelFilter, SetLoggerError};
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};

static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LevelFilter::Info as usize);

pub fn init() -> Result<(), SetLoggerError> {
    let level = env::var("RUST_LOG")
        .ok()
        .and_then(|lvl| lvl.parse::<LevelFilter>().ok())
        .unwrap_or(LevelFilter::Info);

    set_level(level);
    log::set_logger(Box::leak(Box::new(crate::core::DynLogger)))
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

pub fn set_level(level: LevelFilter) {
    LOG_LEVEL.store(level as usize, Ordering::Relaxed);
}

pub fn current_level_filter() -> LevelFilter {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        x if x <= LevelFilter::Off as usize => LevelFilter::Off,
        x if x == LevelFilter::Error as usize => LevelFilter::Error,
        x if x == LevelFilter::Warn as usize => LevelFilter::Warn,
        x if x == LevelFilter::Info as usize => LevelFilter::Info,
        x if x == LevelFilter::Debug as usize => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}


pub struct AnsiColors {
    pub reset: &'static str,
    pub gray: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub green: &'static str,
    pub cyan: &'static str,
}

impl AnsiColors {
    pub fn color(&self, level: Level) -> &'static str {
        let enabled = atty::is(atty::Stream::Stdout);
        if !enabled {
            return "";
        }
        match level {
            Level::Error => self.red,
            Level::Warn => self.yellow,
            Level::Info => self.green,
            Level::Debug => self.cyan,
            Level::Trace => self.reset,
        }
    }
}

pub const COLORS: AnsiColors = AnsiColors {
    reset: "\x1b[0m",
    gray: "\x1b[90m",
    red: "\x1b[31m",
    yellow: "\x1b[33m",
    green: "\x1b[32m",
    cyan: "\x1b[36m",
};
