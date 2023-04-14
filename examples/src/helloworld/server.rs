use rand::distributions::Alphanumeric;
use rand::Rng;
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

fn get_large_string() -> String {
    // 10 megabytes
    rand::thread_rng().sample_iter(&Alphanumeric).take(100_000_000).map(char::from).collect()
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello_small(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request on SayHelloSmall from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Normal Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn say_hello_large(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a normal request on SayHelloLarge from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Very large response: {}", get_large_string()),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
