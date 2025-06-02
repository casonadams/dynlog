#[cfg(feature = "file-trigger")]
use {std::collections::HashSet, std::fs, std::path::Path, std::thread, std::time::Duration};

#[cfg(feature = "file-trigger")]
pub fn enable_file_trigger() {
    let crate_name = env!("CARGO_CRATE_NAME").to_string();

    thread::spawn(move || {
        let mut last_level = crate::current_level().to_string();
        let mut seen_files: HashSet<String> = HashSet::new();

        loop {
            for level in ["error", "warn", "info", "debug", "trace"] {
                let path = format!("/tmp/{}-{}", crate_name, level);

                if Path::new(&path).exists() {
                    if !seen_files.contains(&path) {
                        if let Ok(parsed) = level.parse::<log::LevelFilter>() {
                            if level != last_level {
                                crate::set_level(parsed);
                                eprintln!(
                                    "{}dynlog: log level changed to {}{}",
                                    crate::level::COLORS.gray,
                                    level.to_uppercase(),
                                    crate::level::COLORS.reset
                                );
                                last_level = level.to_string();
                            }
                            seen_files.insert(path.clone());
                            let _ = fs::remove_file(&path);
                        }
                    }
                } else {
                    seen_files.remove(&path);
                }
            }

            thread::sleep(Duration::from_millis(750));
        }
    });
}
