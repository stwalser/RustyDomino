use crate::clients;
use crate::common::protocol::{ClientAnswer, GameCommand};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub struct RemoteReceiver {
    web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl RemoteReceiver {
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { web_socket_stream }
    }
}

#[async_trait]
impl clients::Receiver for RemoteReceiver {
    async fn receive_command(&mut self) -> GameCommand {
        let response = match self.web_socket_stream.next().await.unwrap() {
            Ok(Message::Text(string)) => string,
            Ok(_) | Err(_) => panic!("The server command is shit"),
        };
        serde_json::from_str::<GameCommand>(&response).unwrap()
    }

    async fn respond(&mut self, client_answer: ClientAnswer) {
        let message = serde_json::to_string(&client_answer).unwrap();
        self.web_socket_stream
            .send(Message::Text(message.into()))
            .await
            .unwrap();
    }
}
