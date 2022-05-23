use std::io::Write;

use env_logger::fmt;

use super::handlers;

fn log_level(matches: &clap::ArgMatches) -> log::Level {
    if matches.is_present("quiet") {
        log::Level::Error
    } else {
        match matches.occurrences_of("verbose") {
            // ArgMatches::occurrences_of which will return 0 if the argument was not used at
            // runtime. This demo always displays error or warning messages, so by default -v is
            // always used. The --quiet option must be used to silence all.
            // _ => log::Level::Error,
            // _ => log::Level::Warn,
            0 | 1 => log::Level::Info,
            2 => log::Level::Debug,
            _ => log::Level::Trace, // 3 | _
        }
    }
}

pub fn setup(matches: &clap::ArgMatches) {
    let lvl = log_level(matches);

    env_logger::Builder::new()
        .format(move |f, record| {
            // Color::White renders as gray on black background terminals
            let mut s = f.style();
            let (lvl_str, s) = match record.level() {
                log::Level::Error => ("<e>", s.set_bold(true).set_color(fmt::Color::Red)),
                log::Level::Warn => ("<w>", s.set_bold(true).set_color(fmt::Color::Yellow)),
                log::Level::Info => ("<i>", s.set_bold(false).set_color(fmt::Color::White)),
                log::Level::Debug => ("<d>", s.set_bold(false).set_color(fmt::Color::Blue)),
                log::Level::Trace => ("<t>", s.set_bold(false).set_color(fmt::Color::Magenta)),
            };

            let (target, tstamp) = match lvl {
                l if l >= log::Level::Debug => (record.module_path(), Some(f.timestamp_millis())),
                _ => (None, None), // f.timestamp_seconds()),
            };

            if let Some(tstamp) = tstamp {
                write!(f, "{} {}", s.value(tstamp), s.value(lvl_str))?;
            }

            if let Some(target) = target {
                write!(f, " {}", target)?;
            }
            writeln!(f, " {}", s.value(record.args()))
        })
        .filter_level(lvl.to_level_filter())
        .init();

    if lvl >= log::Level::Debug {
        std::env::set_var("RUST_SPANTRACE", "1");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    color_eyre::config::HookBuilder::default()
        .panic_message(handlers::PanicMessage)
        .display_env_section(std::env::var("DISPLAY_LOCATION").is_ok())
        .install()
        .unwrap();
}
