use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static GLOBAL_RT: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread() // or current_thread if you prefer
        .worker_threads(2) // tweak to your liking
        .enable_all() // reactor + timer
        .build()
        .expect("failed to build global Tokio runtime")
});
