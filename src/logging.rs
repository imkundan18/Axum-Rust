
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
//use chrono::Local;

pub fn init_logging() {
    // Create a file appender with rotation
    let file_appender = RollingFileAppender::new(Rotation::NEVER, "src", "app.log");
    
    // Create a tracing subscriber with the file appender
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false); // Disable ANSI colors in logs
        


    // Create the tracing subscriber and set it as the global default
    let subscriber = Registry::default().with(file_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up global subscriber");
}
