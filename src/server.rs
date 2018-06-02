extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use protos::snake::{PingRequest, PingResponse};
use protos::snake_grpc::{self, Snake};

#[derive(Clone)]
struct SnakeService;

impl Snake for SnakeService {
    fn ping(&self, ctx: RpcContext, ping: PingRequest, sink: UnarySink<PingResponse>) {
        println!("Received ping {{ {:?} }}", ping);
        let mut res = PingResponse::new();
        res.set_version("0.0.1".to_ascii_lowercase());
        let f = sink.success(res.clone())
            .map(move |_| println!("Responded with res {{ {:?} }}", res))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = snake_grpc::create_snake(SnakeService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
