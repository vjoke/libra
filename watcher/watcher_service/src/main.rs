use watcher_service::watcher_node;
use executable_helpers::helpers::{
    setup_executable, ARG_CONFIG_PATH, ARG_DISABLE_LOGGING, ARG_PEER_ID,
};

/// Run a watcher service in its own process.
/// It will also setup global logger and initialize config
fn main() {
    println!("Hello, watcher!");
    let watcher_node = watcher_node::WatcherNode::new();

    watcher_node
        .run()
        .expect("Unable to run watch node");
}
