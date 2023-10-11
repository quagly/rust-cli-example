use anyhow::{Context, Result};
use log::{debug, error, info, trace, warn};
use log4rs;
use rust_cli_example::logmod;

// TODO log panics.  Probably use log_panics crate
// https://docs.rs/log-panics/latest/log_panics/#structs
// should this be a path instead of string slice?
// lets make a config struct with defaults for bootstrap settings
const LOGGING_CONFIG_FILE: &str = "logging-config.yml"; 

fn main() -> Result<()> {
    log4rs::init_file(LOGGING_CONFIG_FILE, Default::default()).unwrap();
    // log4rs::init_file("logging-confi.yml", Default::default())
    //    .with_context(|| format!("Failed to initialize logging from config file {}", file_name))?;
    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning this program doesn't do much");
    error!("error message here");
    for i in 0..5 {
        info!("To test rolling file configurations we print this message in a loop. This is loop nr. {}", i);
    }
    logmod::run();
    
    Ok(())
}
