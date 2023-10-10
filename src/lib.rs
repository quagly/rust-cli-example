pub use log::{debug, error, info, trace, warn};
pub mod logmod {
    use super::*; // this just brings the log:: use into scope.  
    pub fn run() {
        warn!("warn from module");
        info!("info from module");
        debug!("debug from module");
    }
}

