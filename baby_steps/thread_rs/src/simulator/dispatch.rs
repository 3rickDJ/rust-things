use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use crate::message::ProcessMessage;
pub struct Dispatcher {
  sender: mpsc::Sender<ProcessMessage>,
  receiver: mpsc::Receiver<ProcessMessage>
}

impl Dispatcher {
  pub fn new(receiver: Receiver<ProcessMessage> , sender: Sender<ProcessMessage>) -> Dispatcher {
    let dispatcher = Dispatcher {
      sender,
      receiver
    };
    dispatcher
  }

  pub fn run(self) -> thread::JoinHandle<()> {
    thread::spawn( move || {
      loop {
        let message = self.receiver.recv().unwrap();
        match message {
          ProcessMessage::Process(process) => {
            self.sender.send(ProcessMessage::Process(process)).unwrap();
          },
          ProcessMessage::Quit => {
            println!("Fin de la simulación.");
            self.sender.send(ProcessMessage::Quit).unwrap();
            break;
          }
        }
      }
    })
  }
}