use futures::{sync::oneshot, Future};
use std::{sync::Arc, thread};

use crate::{watcher_node, watcher_service::GreeterService};
use grpc_helpers::spawn_service_thread;
use watcher_proto::proto::{
    helloworld::{HelloReply, HelloRequest},
    helloworld_client::GreeterClientTrait,
    helloworld_grpc::{self, GreeterClient},
};

use grpcio::{ChannelBuilder, EnvBuilder};

/// Run a watcher service in its own process.
/// It will also setup global logger and initialize config

fn run_server() {
    println!("Hello, watcher!");
    let watcher_node = watcher_node::WatcherNode::new();

    watcher_node.run().expect("Unable to run watch node");
}

#[test]
fn test_say_hello() {
    let handle = GreeterService::new();
    let port = 12345;

    // run server, note that server will be request to shutdown when server handle
    // returned is dropped
    let service = helloworld_grpc::create_greeter(handle);
    let _greet_service_handle =
        spawn_service_thread(service, "0.0.0.0".to_string(), port, "watcher");

    // run client
    let env = Arc::new(EnvBuilder::new().build());
    let url = format!("0.0.0.0:{}", port);
    let url = &url[..];
    let ch = ChannelBuilder::new(env).connect(url);
    let client = GreeterClient::new(ch);
    let req = HelloRequest::new();

    // let reply = client.say_hello(&req).unwrap();
    // assert_eq!("ray", reply.get_message());

    // get response async
    let reply = <GreeterClient as GreeterClientTrait>::say_hello_async(&client, &req);
    reply
        .map(|f| {
            f.wait().map(|msg| {
                assert_eq!("ray", msg.get_message());
            })
        })
        .map_err(|e| panic!("should not happen"));
}

#[test]
fn test_sync() {
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("begin to wait in child thread");
        rx.map(|x| println!("got {}", x)).wait();
    });

    println!("send command in main thread");
    tx.complete(100);
}

#[test]
fn test_box() {
    let b1 = Box::new("hello");
    assert_eq!(*b1, "hello");
}

#[test]
fn test_box_trait() {
    use failure;
    trait Tool {
        fn order_food(&self) -> Result<(), Box<std::error::Error>>;
        fn get_food(&self) -> Result<(), failure::Error>;
    }

    struct MyTool{};
    impl Tool for MyTool{
        fn order_food(&self) -> Result<(), Box<std::error::Error>> {
            Ok(())
        }

        fn get_food(&self) -> Result<(), failure::Error> {
            Ok(())
        }
    }

    let mt = MyTool{};
    assert_eq!((), mt.get_food().unwrap());
}
