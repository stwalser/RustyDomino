use async_trait::async_trait;
use crate::common::protocol::{ClientAnswer, GameCommand};
use std::sync::mpsc::{Receiver, SyncSender};
use crate::clients;

#[derive(Debug)]
pub struct LocalReceiver {
    command_receiver: Receiver<GameCommand>,
    answer_sender: SyncSender<ClientAnswer>,
}

impl LocalReceiver {
    pub fn new(
        command_receiver: Receiver<GameCommand>,
        answer_sender: SyncSender<ClientAnswer>,
    ) -> Self {
        Self {
            command_receiver,
            answer_sender,
        }
    }
}

#[async_trait]
impl clients::Receiver for LocalReceiver {
    async fn receive_command(&mut self) -> GameCommand {
        self.command_receiver.recv().unwrap()
    }

    async fn respond(&mut self, client_answer: ClientAnswer) {
        self.answer_sender.send(client_answer).unwrap()
    }
}
