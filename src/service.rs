use std::{
    any::Any,
    sync::atomic::{AtomicU64, Ordering},
};

use tokio::sync::oneshot;

pub type Response = Box<dyn Any + Send + Sync + 'static>;

pub enum Task {
    GetNumber,
}

pub struct Message {
    pub response_channel: oneshot::Sender<Response>,
    pub task: Task,
}

#[derive(Default)]
pub struct ApiService {
    inner_number: AtomicU64,
}

impl ApiService {
    pub async fn handle_task(&self, message: Message) -> Result<(), Response> {
        let response = match message.task {
            Task::GetNumber => self.get_number(),
        };

        message.response_channel.send(response)
    }

    fn get_number(&self) -> Response {
        Box::new(self.inner_number.load(Ordering::Relaxed))
    }
}
