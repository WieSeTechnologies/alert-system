#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

fn main() {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    println!("Hello World!");
}
