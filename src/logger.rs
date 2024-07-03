use std::env;
use tracing::{info, Level};
use tracing_subscriber;

pub fn init() {
    // Configure logging
    let loglevel = match env::var("LOG_LEVEL") {
        Ok(val) => val.parse().unwrap_or(Level::INFO),
        Err(_e) => Level::INFO, // default to Info
    };
    tracing_subscriber::fmt()
        .with_target(false) // turn off file name
        .with_max_level(loglevel) // set max level to log
        //.json() // uncomment this line to convert it into json output
        .init();

    // Log the application banner
    log_app_banner();
}

pub fn log_app_banner() {
    info!(
        "{}",
        format!(
            "<< ===================={{ {} v{} }}==================== >>",
            crate::consts::APP_NAME,
            crate::consts::APP_VER
        )
    );
}
