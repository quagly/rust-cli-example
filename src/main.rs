use anyhow::{Context, Result};
use log::{debug, error, info, trace, warn};
use log_panics;
use log4rs;
// this is just an example of how a module uses the log
// initialized here
use rust_cli_example::logmod;

// TODO log panics.  Probably use log_panics crate
// https://docs.rs/log-panics/latest/log_panics/#structs
// should this be a path instead of string slice?
// lets make a config struct with defaults for bootstrap settings
const LOGGING_CONFIG_FILE: &str = "logging-config.yml"; 

fn main() -> Result<()> {
    log4rs::init_file(LOGGING_CONFIG_FILE, Default::default()).unwrap();
    // backtrace mode options https://docs.rs/log-panics/latest/log_panics/enum.BacktraceMode.html#variants
    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook();
    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning this program doesn't do much");
    error!("error message here");
    for i in 0..5 {
        info!("To test rolling file configurations we print this message in a loop. This is loop nr. {}", i);
    }
    logmod::run();
    // demonstrate log_panics
    // panic!("very bad!");
    
    Ok(())
}
