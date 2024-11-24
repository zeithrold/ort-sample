use anyhow::{Ok, Result};
use fern::{self, colors::ColoredLevelConfig};
use log::{self, info};

pub fn init_log() -> Result<()> {
    let colors = ColoredLevelConfig::new();
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("output.log")?)
        .apply()?;
    info!("Log initialized");
    Ok(())
}
