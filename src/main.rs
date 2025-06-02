use dynlog::init;
use log::{debug, error, info, trace, warn};

#[cfg(feature = "socket-trigger")]
#[tokio::main]
async fn main() {
    init().unwrap();
    dynlog::enable_socket_trigger().await;

    #[cfg(feature = "file-trigger")]
    dynlog::enable_file_trigger();

    run();
}

#[cfg(not(feature = "socket-trigger"))]
fn main() {
    init().unwrap();

    #[cfg(feature = "file-trigger")]
    dynlog::enable_file_trigger();

    run();
}

fn run() {
    loop {
        info!("Service started");
        debug!("Debugging enabled");
        trace!("Tracing enabled");
        warn!("This is a warning");
        error!("Something went wrong");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
