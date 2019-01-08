use std;

use fern::colors::{Color, ColoredLevelConfig};
use log::warn;

/// Setup the logging implementation for use in the library.
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
        0 => base_config.level(log::LevelFilter::Warn),
        1 => base_config.level(log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        3 | _ => base_config.level(log::LevelFilter::Trace),
    };

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            let color_line_start = if no_color {
                String::from("")
            } else {
                format!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                )
            };
            let color_line_end = if no_color {
                String::from("")
            } else {
                String::from("\x1B[0m")
            };

            out.finish(format_args!(
                "{color_line_start}[{date}][{target}][{level}] {message}{color_line_end}",
                color_line_start = color_line_start,
                color_line_end = color_line_end,
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ))
        })
        .chain(std::io::stderr());

    if base_config.chain(stdout_config).apply().is_err() {
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
