use futures::Future;

use crate::watcher_node;

/// Run a watcher service in its own process.
/// It will also setup global logger and initialize config

fn run_server() {
    println!("Hello, watcher!");
    let watcher_node = watcher_node::WatcherNode::new();

    watcher_node
        .run()
        .expect("Unable to run watch node");
}

#[test]
fn test_say_hello() {
    assert_eq!(1, 1);
}