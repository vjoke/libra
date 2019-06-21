use grpc_helpers::provide_grpc_response;

use watcher_proto::proto::{
    helloworld::{
        HelloRequest, HelloReply,
    },
    helloworld_grpc::Greeter,
};

/// Struct implementing trait (service handle) Greeter
#[derive(Clone)]
pub struct GreeterService {

}

impl GreeterService {
    pub fn new() -> Self {
        GreeterService{}
    }
}

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: ::grpcio::RpcContext, req: HelloRequest, sink: ::grpcio::UnarySink<HelloReply>) {
        println!("say_hello called");
        let mut msg = HelloReply::new();
        msg.set_message("ray".to_string());
        let resp = Ok(msg);
        provide_grpc_response(resp, ctx, sink);
    }
}