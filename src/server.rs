use grpc_example::schema::{
    api_server::{Api, ApiServer},
    Empty, HelloThereResponse,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Clone, Copy)]
struct MyApi;

#[tonic::async_trait]
impl Api for MyApi {
    async fn hello_there(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HelloThereResponse>, Status> {
        Ok(Response::new(HelloThereResponse {
            message: "General Kenobi".to_owned(),
        }))
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:5000".parse().expect("Error parsing");

    Server::builder()
        .add_service(ApiServer::new(MyApi))
        .serve(addr)
        .await
        .expect("Error");
}
