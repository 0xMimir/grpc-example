use grpc_example::schema::{
    api_server::{Api, ApiServer},
    Empty, HelloThereResponse,
};
use service::{ApiService, Message, Task};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot,
};
use tonic::{transport::Server, Code, Request, Response, Status};

pub mod service;

struct ApiHandler {
    transmitter: UnboundedSender<Message>,
}

impl ApiHandler {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let service = ApiService::default();

            while let Some(message) = rx.recv().await {
                if let Err(error) = service.handle_task(message).await {
                    log::error!("{:#?}", error);
                }
            }
        });

        Self { transmitter: tx }
    }

    async fn handle_task<T: 'static>(&self, task: Task) -> Result<T, Status> {
        let (tx, rx) = oneshot::channel();

        self.transmitter
            .send(Message {
                response_channel: tx,
                task,
            })
            .expect("Error sending message");

        let response = rx
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?
            .downcast()
            .map_err(|_| Status::new(Code::Internal, "Internal error"))?;

        Ok(*response)
    }
}

#[tonic::async_trait]
impl Api for ApiHandler {
    async fn hello_there(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HelloThereResponse>, Status> {
        let response: u64 = self.handle_task(Task::GetNumber).await?;
        Ok(Response::new(HelloThereResponse {
            message: response.to_string(),
        }))
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:5000".parse().expect("Error parsing");

    Server::builder()
        .add_service(ApiServer::new(ApiHandler::new()))
        .serve(addr)
        .await
        .expect("Error");
}
