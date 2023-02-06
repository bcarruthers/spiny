#![forbid(unsafe_code)]

use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[cfg(tracy)]
mod tracy;

pub fn init() {
    // Get filters from env var
    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env());
    // Create combined subscriber
    let subscriber = tracing_subscriber::registry().with(console_layer);
    #[cfg(tracy)]
    let subscriber = subscriber.with(tracy::start_tracy());
    // Set globals
    LogTracer::init().expect("Could not set log tracer");
    tracing::subscriber::set_global_default(subscriber).expect("Could not set global logger");
}
