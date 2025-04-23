use chrono::Utc;
use std::{
    fs::{self, OpenOptions, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self, time::ChronoUtc},
    prelude::*,
    EnvFilter
};

pub fn init(log_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    ensure_logs_folder_exists(log_folder)?;
    
    let log_file = get_log_file(log_folder)?;
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let file_layer = fmt::Layer::new()
        .json()
        .with_timer(ChronoUtc::rfc_3339())
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(move || BufWriter::new(log_file.try_clone().unwrap()));
    
    let console_layer = fmt::Layer::new()
        .with_timer(ChronoUtc::rfc_3339())
        .with_target(true)
        .with_ansi(true);
    
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer);
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    LogTracer::init()?;
    
    Ok(())
}

fn ensure_logs_folder_exists(folder_path: &str) -> io::Result<()> {
    let path = Path::new(folder_path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn get_log_file(folder: &str) -> io::Result<File> {
    let date_str = Utc::now().format("%Y-%m-%d").to_string();
    let log_path = PathBuf::from(folder).join(format!("{}.log", date_str));
    OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(log_path)
}
