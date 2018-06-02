// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SNAKE_PING: ::grpcio::Method<super::snake::PingRequest, super::snake::PingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Snake/Ping",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SNAKE_JOIN: ::grpcio::Method<super::snake::ClientCommand, super::snake::ServerCommand> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/Snake/Join",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct SnakeClient {
    client: ::grpcio::Client,
}

impl SnakeClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SnakeClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn ping_opt(&self, req: &super::snake::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::snake::PingResponse> {
        self.client.unary_call(&METHOD_SNAKE_PING, req, opt)
    }

    pub fn ping(&self, req: &super::snake::PingRequest) -> ::grpcio::Result<super::snake::PingResponse> {
        self.ping_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ping_async_opt(&self, req: &super::snake::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::snake::PingResponse>> {
        self.client.unary_call_async(&METHOD_SNAKE_PING, req, opt)
    }

    pub fn ping_async(&self, req: &super::snake::PingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::snake::PingResponse>> {
        self.ping_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn join_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::snake::ClientCommand>, ::grpcio::ClientDuplexReceiver<super::snake::ServerCommand>)> {
        self.client.duplex_streaming(&METHOD_SNAKE_JOIN, opt)
    }

    pub fn join(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::snake::ClientCommand>, ::grpcio::ClientDuplexReceiver<super::snake::ServerCommand>)> {
        self.join_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Snake {
    fn ping(&self, ctx: ::grpcio::RpcContext, req: super::snake::PingRequest, sink: ::grpcio::UnarySink<super::snake::PingResponse>);
    fn join(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::snake::ClientCommand>, sink: ::grpcio::DuplexSink<super::snake::ServerCommand>);
}

pub fn create_snake<S: Snake + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SNAKE_PING, move |ctx, req, resp| {
        instance.ping(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_SNAKE_JOIN, move |ctx, req, resp| {
        instance.join(ctx, req, resp)
    });
    builder.build()
}
