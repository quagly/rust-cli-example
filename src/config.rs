use super::*; // this just brings the log:: use into scope.  
 
// configuration sufficient to get additional configuration from file
// TODO - consider using Path instead of String for files
#[derive(Debug)]
pub struct Bootstrap {
    // need to initialize logging before reading global configuration
    // so that problems reading configuation can be logged
    logging_configuration_filename: String,
    // everything else should be in the config file
    global_configuration_filename: String,
}

impl Default for Bootstrap {
    fn default() -> Bootstrap {
        Bootstrap {
            logging_configuration_filename: String::from("logging-config.yml"), 
            global_configuration_filename: String::from("config.yml")
        }
    }
}


pub fn run() {
    let default_bootstrap: Bootstrap = Default::default(); 
    debug!("default bootstrap config is {:#?}", default_bootstrap);
}
