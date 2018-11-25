extern crate chrono;
extern crate fern;
extern crate log;

use std;

#[cfg(unix)]
use fern::colors::{Color, ColoredLevelConfig};

/// Setup the logging implementation for use in the library.
#[cfg(unix)]
pub fn setup_logging(verbosity: u8, no_color: bool) -> Result<(), ()> {
    // configure colors for the whole line
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they
        // are white by default
        .info(Color::Cyan)
        .debug(Color::BrightBlue)
        // depending on the terminals color scheme, this is the same as the
        // background color
        .trace(Color::BrightCyan);

    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config
            .level(log::LevelFilter::Warn)
            .level_for("tokio", log::LevelFilter::Error)
            .level_for("reqwest", log::LevelFilter::Error)
            .level_for("tokio_reactor", log::LevelFilter::Error)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Error)
            .level_for("want", log::LevelFilter::Error)
            .level_for("mio", log::LevelFilter::Error),
        1 => base_config
            .level(log::LevelFilter::Info)
            .level_for("tokio", log::LevelFilter::Warn)
            .level_for("reqwest", log::LevelFilter::Warn)
            .level_for("tokio_reactor", log::LevelFilter::Warn)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Warn)
            .level_for("want", log::LevelFilter::Warn)
            .level_for("mio", log::LevelFilter::Warn),
        2 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("tokio", log::LevelFilter::Info)
            .level_for("reqwest", log::LevelFilter::Info)
            .level_for("tokio_reactor", log::LevelFilter::Info)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Info)
            .level_for("want", log::LevelFilter::Info)
            .level_for("mio", log::LevelFilter::Info),
        3 | _ => base_config
            .level(log::LevelFilter::Trace)
            .level_for("tokio", log::LevelFilter::Info)
            .level_for("reqwest", log::LevelFilter::Info)
            .level_for("tokio_reactor", log::LevelFilter::Info)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Info)
            .level_for("want", log::LevelFilter::Info)
            .level_for("mio", log::LevelFilter::Info),
    };

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            let mut color_line_start = format!(
                "\x1B[{}m",
                colors_line.get_color(&record.level()).to_fg_str()
            );
            let mut color_line_end = String::from("\x1B[0m");

            // Clear colors if flag is set
            if no_color {
                color_line_start = String::from("");
                color_line_end = String::from("");
            }

            out.finish(format_args!(
                "{color_line_start}[{date}][{target}][{level}] {message}{color_line_end}",
                color_line_start = color_line_start,
                color_line_end = color_line_end,
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ))
        }).chain(std::io::stderr());

    if let Err(_) = base_config.chain(stdout_config).apply() {
        // Still return Ok, but warn the user.
        warn!("Logger was already set and cannot be reset.");
    }
    Ok(())
}

#[cfg(windows)]
pub fn setup_logging(verbosity: u8, _no_color: bool) -> Result<()> {
    let mut base_config = fern::Dispatch::new();
    base_config = match verbosity {
        0 => base_config
            .level(log::LevelFilter::Warn)
            .level_for("tokio", log::LevelFilter::Error)
            .level_for("reqwest", log::LevelFilter::Error)
            .level_for("tokio_reactor", log::LevelFilter::Error)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Error),
        1 => base_config
            .level(log::LevelFilter::Info)
            .level_for("tokio", log::LevelFilter::Warn)
            .level_for("reqwest", log::LevelFilter::Warn)
            .level_for("tokio_reactor", log::LevelFilter::Warn)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Warn),
        2 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("tokio", log::LevelFilter::Info)
            .level_for("reqwest", log::LevelFilter::Info)
            .level_for("tokio_reactor", log::LevelFilter::Info)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Info),
        3 | _ => base_config
            .level(log::LevelFilter::Trace)
            .level_for("tokio", log::LevelFilter::Debug)
            .level_for("reqwest", log::LevelFilter::Debug)
            .level_for("tokio_reactor", log::LevelFilter::Debug)
            .level_for("html5ever", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Debug),
    };

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ))
        }).chain(std::io::stderr());

    if let Err(_) = base_config.chain(stdout_config).apply() {
        // Still return Ok, but warn the user.
        warn!("Logger was already set and cannot be reset.");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_log_level_5() {
        setup_logging(5, false).unwrap();
    }
    #[test]
    fn test_set_log_level_1() {
        setup_logging(1, false).unwrap();
    }
}
