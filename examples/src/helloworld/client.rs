use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::transport::Channel;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

async fn get_client() -> GreeterClient<Channel> {
    let channel = Channel::builder("http://localhost:50051".parse().unwrap())
        .connect()
        .await
        .unwrap();
    GreeterClient::new(channel)
}

fn get_request() -> tonic::Request<HelloRequest> {
    tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Invoking both SayHelloSmall and SayHelloLarge on a default client
    {
        let mut default_client = get_client().await;
        let default_small_response = default_client.say_hello_small(get_request()).await;
        println!("Default client on SayHelloSmall={:?}", default_small_response);
        let default_large_response = default_client.say_hello_large(get_request()).await;
        println!("Default client on SayHelloLarge={:?}", default_large_response);
    }

    // Doing the same, but on a client that specifies max_decoding_size
    {
        let mut custom_client = get_client().await.max_decoding_message_size(usize::MAX);
        let custom_small_response = custom_client.say_hello_small(get_request()).await;
        println!("Custom client on SayHelloSmall={:?}", custom_small_response);
        let custom_large_response = custom_client.say_hello_large(get_request()).await;
        println!("Custom client on SayHelloLarge.is_ok()={:?}", custom_large_response.is_ok());
    }

    Ok(())
}
