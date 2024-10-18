use std::fmt;

use color_eyre::owo_colors::OwoColorize;
pub struct PanicMessage;

impl color_eyre::section::PanicMessage for PanicMessage {
    fn display(
        &self,
        pi: &std::panic::PanicHookInfo<'_>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(f, "\nWell, this is embarrassing.\n")?;
        writeln!(
            f,
            "{}",
            format!(
                "{} had a problem and crashed. Consider reporting the bug at {}.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_REPOSITORY")
            )
            .red()
        )?;

        writeln!(
            f,
            "\nWe take privacy seriously, and do not perform any \
           automated error collection. In order to improve the software, we rely on \
           people to submit reports.\n"
        )?;
        writeln!(f, "Thank you kindly!\n\n---\n")?;

        // Print panic message.
        let payload = pi
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| pi.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<non string panic payload>");

        write!(f, "Message:  ")?;
        writeln!(f, "{}", payload.cyan())?;

        // panic location.
        write!(f, "Location: ")?;

        if let Some(loc) = pi.location() {
            write!(f, "{}", loc.file().purple())?;
            write!(f, ":")?;
            write!(f, "{}", loc.line().purple())?;
        } else {
            write!(f, "<unknown>")?;
        }
        writeln!(f)?;

        Ok(())
    }
}
