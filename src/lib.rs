pub use log::{debug, error, info, trace, warn};
//  why did I create a submodule of rust-log?  rust-log::logmod
pub mod logmod {
    use super::*; // this just brings the log:: use into scope.  
    pub fn run() {
        warn!("warn from module");
        info!("info from module");
        debug!("debug from module");
    }
}

