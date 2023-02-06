fn is_tracy_enabled() -> bool {
    if std::env::args().any(|s| s.contains("tracy")) {
        true
    } else {
        match std::env::var("RUST_TRACY") {
            Ok(value) => value == "1",
            Err(_) => false,
        }
    }
}

/// Create tracy client if enabled
pub fn start_tracy() -> Option<tracing_tracy::TracyLayer> {
    if is_tracy_enabled() {
        // This will start the client and send local discovery packets
        let filter: EnvFilter = "debug".parse().unwrap();
        let layer = tracing_tracy::TracyLayer::new().with_filter(filter);
        tracing::info!("Tracy client enabled");
        Some(layer)
    } else {
        None
    }
}
