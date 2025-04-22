use fern::Dispatch;
use log::LevelFilter;
use chrono::Local;
use colored::*;
use std::fs;
use std::path::Path;
use regex;

fn ensure_logs_folder_exists(folder_path: &str) -> std::io::Result<()> {
    let path = Path::new(folder_path);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn init(debug: bool, path: &str) -> Result<(), std::io::Error> {
    let log_level = if debug { LevelFilter::Debug } else { LevelFilter::Info };
    let _ = ensure_logs_folder_exists(path);

    let stdout_formatter = |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
        let level_color = match record.level() {
            log::Level::Error => "ERROR".red(),
            log::Level::Warn  => "WARN".yellow(),
            log::Level::Info  => "INFO".green(),
            log::Level::Debug => "DEBUG".blue(),
            log::Level::Trace => "TRACE".purple(),
        };

        out.finish(format_args!(
            "[{} {} {}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            level_color,
            record.target().cyan(),
            message
        ))
    };

    let file_formatter = |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
        out.finish(format_args!(
            "{{\"timestamp\":\"{}\",\"level\":\"{}\",\"target\":\"{}\",\"region\":\"{}\",\"latency\":{} }}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            extract_region(message),
            extract_latency(message)
        ))
    };

    fn extract_region(message: &std::fmt::Arguments) -> String {
        let regex = regex::Regex::new(r"([A-Za-z0-9 ]+)\.{2,}").unwrap();
        if let Some(caps) = regex.captures(message.to_string().as_str()) {
            caps.get(1).map_or_else(|| String::from("Unknown"), |m| m.as_str().to_string())
        } else {
            String::from("Unknown")
        }
    }

    fn extract_latency(message: &std::fmt::Arguments) -> u32 {
        let regex = regex::Regex::new(r"(\d+)ms").unwrap();
        if let Some(caps) = regex.captures(message.to_string().as_str()) {
            caps.get(1).map_or_else(|| 0, |m| m.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }

    match Dispatch::new()
        .level(log_level)
        .chain(
            Dispatch::new()
                .format(stdout_formatter)
                .chain(std::io::stdout())
        )
        .chain(
            Dispatch::new()
                .format(file_formatter)
                .chain(fern::log_file(format!("{}/{}.log", path, Local::now().format("%Y-%m-%d")))?)
        )
        .apply() {
            Ok(()) => {
                Ok(())
            },
            Err(e) => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            }
        }
}
