// this just brings the log:: use into scope.  
use super::*; 
 
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

#[cfg(test)]
mod tests {
    use super::*;
    use all_asserts::assert_false;
    use predicates::prelude::*;

    #[test]
    // test that log config file name has a default set to something 
    // using all_asserts crate
    fn default_log_config_set() {
        let default_bootstrap: Bootstrap = Default::default(); 
        assert_false!(default_bootstrap.logging_configuration_filename.trim().is_empty())
    }

    #[test]
    fn default_log_config_is_set() {
        let default_bootstrap: Bootstrap = Default::default(); 
        let pred_string_not_set = predicate::str::is_empty().trim().name("trimmed_string_is_empty");
        assert_false!(pred_string_not_set.eval(&default_bootstrap.logging_configuration_filename));
    }
}
