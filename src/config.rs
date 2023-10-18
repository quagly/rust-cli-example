// this just brings the log:: use into scope.  
use super::*; 
use std::any::type_name;
 
// types are known at compile time so need a function to capture them at compile time
// TODO remove allow dead code
#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// configuration sufficient to get additional configuration from file
// TODO - consider using Path instead of String for files
// TODO - remove allow dead_code
#[derive(Debug)]
#[allow(dead_code)]
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
    use static_assertions::assert_fields;

    #[test]
    // test that log bootstrap has all required fields 
    // just an example of using assert_fields! macro.  
    // unneccessary here as what fields there are is not dynamic
    // so the compiler will check them
    fn default_config_fields() {
        assert_fields!(Bootstrap: logging_configuration_filename,global_configuration_filename);
    }

    #[test]
    // test that log config file name has a default set to something 
    // using all_asserts crate
    fn default_log_config_set() {
        let default_bootstrap: Bootstrap = Default::default(); 
        assert_false!(default_bootstrap.logging_configuration_filename.trim().is_empty())
    }

    #[test]
    // test that log config file name has a default set to something - same as above
    // using predicate crate
    // I think I would like predicates like this if I could reuse them.  But I do not see how.
    fn default_log_config_is_set() {
        let default_bootstrap: Bootstrap = Default::default(); 
        // this predicate seems generally useful.  How to make it const or static?  
        let pred_string_not_set = predicate::str::is_empty().trim().name("trimmed_string_is_empty");
        assert_false!(pred_string_not_set.eval(&default_bootstrap.logging_configuration_filename));
    }

    #[test]
    // test that global config file name has a default set to something 
    fn default_global_config_is_set() {
        let default_bootstrap: Bootstrap = Default::default(); 
        let pred_string_not_set = predicate::str::is_empty().trim().name("trimmed_string_is_empty");
        assert_false!(pred_string_not_set.eval(&default_bootstrap.global_configuration_filename));
    }
}
