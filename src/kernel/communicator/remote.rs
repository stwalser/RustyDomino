use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::kernel::communicator::Sender;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::fmt::{Debug, Formatter};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

pub struct Remote {
    web_socket_stream: WebSocketStream<TcpStream>,
}

impl Remote {
    pub fn new(web_socket_stream: WebSocketStream<TcpStream>) -> Self {
        Self { web_socket_stream }
    }
}

impl Debug for Remote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Remote").finish()
    }
}

#[async_trait]
impl Sender for Remote {
    async fn send_command(&mut self, game_command: GameCommand) -> ClientAnswer {
        let message = serde_json::to_string(&game_command).unwrap();
        self.web_socket_stream
            .send(Message::Text(message.into()))
            .await
            .unwrap();
        let response = match self.web_socket_stream.next().await.unwrap() {
            Ok(Message::Text(string)) => string,
            Ok(_) | Err(_) => panic!("The client response is shit"),
        };
        serde_json::from_str::<ClientAnswer>(&response).unwrap()
    }
}
