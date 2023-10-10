use anyhow::{Context, Result};
use log::{debug, error, info, trace, warn};
use log4rs;
use rust_cli_example::logmod;

fn main() -> Result<()> {
    log4rs::init_file("logging-config.yml", Default::default()).unwrap();
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
