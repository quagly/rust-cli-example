// get all use from parent
// need log exports
use super::*; // this just brings the log:: use into scope.  
pub fn run() {
    warn!("warn from logmod module");
    info!("info from logmod module");
    debug!("debug from logmod module");
}
