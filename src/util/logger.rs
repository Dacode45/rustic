use chrono;
use fern;
use log;
use std;

pub fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors = ColoredLevelConfig::default()
        .info(Color::Green)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightBlue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<14}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()).to_string(),
                record.target(),
                message
            ))
        })
        // gfx_device_gl is very chatty on info loglevel, so
        // filter that a bit more strictly.
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(std::fs::OpenOptions::new()
               .write(true)
               .create(true)
               .truncate(true)
               .open("debug.log")?)
        .apply()?;
    Ok(())
}
