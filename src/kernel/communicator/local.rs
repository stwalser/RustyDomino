use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::kernel::communicator::Communicator;
use std::fmt;
use std::sync::mpsc::{Receiver, SyncSender};

pub struct Local {
    command_sender: SyncSender<GameCommand>,
    answer_receiver: Receiver<ClientAnswer>,
}

impl fmt::Debug for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Local").finish()
    }
}

impl Local {
    pub fn new(
        command_sender: SyncSender<GameCommand>,
        answer_receiver: Receiver<ClientAnswer>,
    ) -> Self {
        Self {
            command_sender,
            answer_receiver,
        }
    }
}

impl Communicator for Local {
    fn send_command(&self, game_command: GameCommand) -> ClientAnswer {
        self.command_sender
            .send(game_command)
            .unwrap();
        self.answer_receiver.recv().unwrap()
    }
}
