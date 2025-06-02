use dynlog::init;
use log::{debug, error, info, trace, warn};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[cfg(feature = "socket-trigger")]
// #[tokio::main]
// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
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
        let sys = System::new_all();

        if let Some(process) = sys.process(sysinfo::get_current_pid().unwrap()) {
            info!("Service started");
            debug!("Debugging enabled");
            trace!("Tracing enabled");
            warn!("This is a warning");
            error!("Something went wrong");

            info!("CPU usage: {:.2}%", process.cpu_usage());
            info!("Memory: {} KB", process.memory());
            info!("Virtual Memory: {} KB", process.virtual_memory());
            info!(
                "Open file descriptors: {}",
                process.open_files().map_or(0, |f| f.len())
            );
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
