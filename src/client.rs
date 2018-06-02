extern crate grpcio;
extern crate protos;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::snake::{PingRequest};
use protos::snake_grpc::SnakeClient;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the port to connect to.")
    }
    let port = args[1]
        .parse::<u16>()
        .expect(format!("{} is not a valid port number", args[1]).as_str());

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = SnakeClient::new(ch);

    let mut ping = PingRequest::new();

    ping.set_version("v0.0.1".to_ascii_lowercase());

    let res = client.ping(&ping).expect("RPC Failed!");
    println!("req {:?} res: ${:?}", ping, res);
}
