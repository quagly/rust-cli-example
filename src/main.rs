use anyhow::{Context, Result};
use log4rs;
use log_panics;
use rust_cli_example::*;

static CONFIG: OnceLock<config::Config> = OnceLock::new();

// TODO config from yaml file function

fn main() -> Result<()> {
    // get bootstrap defaults to initialize logging and config
    let default_bootstrap: config::Bootstrap = Default::default();
    // log4rs::init_file(LOGGING_CONFIG_FILE, Default::default()).unwrap();
    log4rs::init_file(default_bootstrap.logging_configuration_filename, Default::default()).unwrap();
    // backtrace mode options https://docs.rs/log-panics/latest/log_panics/enum.BacktraceMode.html#variants
    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook();
    // try out logging
    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning this program doesn't do much");
    error!("error message here");
    for i in 0..5 {
        info!("To test rolling file configurations we print this message in a loop. This is loop nr. {}", i);
    }
    logmod::run();
    config::run();
    // demonstrate log_panics
    // panic!("very bad!");
    
    // initialize config from yaml
    let config_file = std::fs::File::open(&default_bootstrap.global_configuration_filename)
        .with_context(|| format!("Failed to read file from {}", default_bootstrap.global_configuration_filename))?;
    info!("config_file type is: {}", config::type_of(&config_file));
    let config_from_file: config::Config = serde_yaml::from_reader(config_file)
        .with_context(|| format!("Failed to initialize Config"))?;
    info!("config from file is: {:?}", config_from_file);

    let initialize_config_result = CONFIG.set(config_from_file);
    assert!(initialize_config_result.is_ok());

    // use CONFIG
    info!("update_frequency_sec is: {}", CONFIG.get().unwrap().update_frequency_sec);
    info!("OnceLock CONFIG is: {:#?}", CONFIG);

    Ok(())
}
