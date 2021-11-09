use tonic::transport::Server;
use tonic::{Request, Response, Status};

use learning_grpc::hello;
use hello::hello_server::{HelloServer, Hello};
use hello::{HelloRequest, HelloResponse};


#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl Hello for MyServer {
    async fn hello_world(&self, _ : Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse { message: "Hello, World!".to_string() };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    let hello_server = MyServer::default();
    Server::builder()
        .add_service(HelloServer::new(hello_server))
        .serve(addr)
        .await?;

    Ok(())
}