use clap::{crate_version, Command};
use std::env;
use uucore::format_usage;

use uucore::error::{UResult};


static ABOUT: &str = "";
const USAGE: &str = ".";

#[uucore::main]
pub fn uumain(_: impl uucore::Args) -> UResult<()> {
    Ok(())
}

pub fn uu_app<'a>() -> Command<'a> {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
}