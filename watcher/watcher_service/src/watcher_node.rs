use crate::watcher_service::GreeterService;
use watcher_proto::proto::helloworld_grpc;
use logger::prelude::*;
use grpc_helpers::spawn_service_thread;
use std::{thread};
use failure::prelude::*;

/// Struct to run watcher service in a dedicated process.
pub struct WatcherNode {

}

impl Drop for WatcherNode {
    fn drop(&mut self) {
        println!("Drop watcher node");
    }
}

impl WatcherNode {
    pub fn new() -> Self {
        WatcherNode {}
    }

    pub fn run(&self) -> Result<()> {
        println!("Starting watcher node");

        let handle = GreeterService::new();
        let service = helloworld_grpc::create_greeter(handle);

        let _greet_service_handle = spawn_service_thread(
            service,
            "0.0.0.0".to_string(),
            12345,
            "watcher",
        );

        loop {
            thread::park();
        }
    }
}