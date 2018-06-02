extern crate futures;
extern crate grpcio;
extern crate protos;
extern crate rand;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};
use std::option::Option;
use std::collections::{VecDeque, HashMap};
use std::sync::atomic::AtomicBool;

use futures::sync::oneshot;
use futures::Future;
use futures::sink::{Sink};
use futures::stream::Stream;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, RequestStream, DuplexSink, WriteFlags};

use protos::snake::{PingRequest, PingResponse, ClientCommand, ServerCommand, DirectionCommand_Direction};
use protos::snake_grpc::{self, Snake};

struct SnakeService {
    client_commands: Box<Sink<SinkItem=(u8, ClientCommand), SinkError=grpcio::Error> + Send>,
    server_commands: Box<Stream<Item=ServerCommand, Error=grpcio::Error> + Send>
}

impl Snake for SnakeService {
    fn join(&self, ctx: RpcContext, incoming: RequestStream<ClientCommand>, outgoing: DuplexSink<ServerCommand>) {
        let snake_id = rand::random::<u8>();
        let client_recv = incoming
            .map(|x| (snake_id, x));

        let incoming_f = self.client_commands.send_all(client_recv)
            .map(|_| ())
            .map_err(|_| ());

        let to_send = self.server_commands
            .map(|x| (x, WriteFlags::default()));

        let outgoing_f = outgoing.send_all(to_send)
            .map(|_| ())
            .map_err(|_| ());

        ctx.spawn(Box::new(outgoing_f));
        ctx.spawn(Box::new(incoming_f));

    }
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

impl SnakeService {
    fn new(g: Game) -> SnakeService {
        SnakeService {
            client_commands = g.client_command_sink();
            server_commands = g.server_commands_stream();
        }
    }
}

struct Game {
    server_commands_bfr: Box<VecDeque<ServerCommand>>,
    client_commands_bfr: Box<VecDeque<(u8, ClientCommand)>>
}

impl Game {
    fn server_commands_stream(&self) -> Box<Stream<Item=ServerCommand, Error=grpcio::Error> + Send> {
        server_commands_bfr
    }

    fn client_command_sink(&self) -> Box<Stream<Item=ServerCommand, Error=grpcio::Error> + Send> {
        client_commands_bfr
    }

    fn new() -> Game {
        Game{}
    }

    fn start(&self) -> Future<()> {
        let hm = HashMap::new()
        client_commands_bfr.as_stream()
            .map(|(user, cmd)| (user, cmd.command.get_direction().direction))
            .filter(|(user, direction)| direction != DirectionCommand_Direction.INVALID)
            .foreach(|(user, cmd)| hm.insert(user, direction));
        // combine timer with direction-map

        // Every tick move players according to above
        // For each player:
        // 1) Nothing -> remove last entry of tail
        // 2) Fruit -> incr score, do nothing
        // 3) Tail -> Die
        // 4) Head -> Die
        // Find mismatches between applications of the above algorithm
    }
}



fn main() {
    let env = Arc::new(Environment::new(1));
    let game = Game::new();
    let service = snake_grpc::create_snake(SnakeService::new(game));
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
