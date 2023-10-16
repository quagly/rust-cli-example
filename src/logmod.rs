// get all use from parent
// need log exports
use super::*; // this just brings the log:: use into scope.  
pub fn run() {
    warn!("warn from module");
    info!("info from module");
    debug!("debug from module");
}
