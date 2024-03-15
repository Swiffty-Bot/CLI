mod cli;
mod model;

fn main() {
    // look into `tracing` and `tracing-subscriber` crates for pretty logs
    /*
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
    */

    cli::run();
}