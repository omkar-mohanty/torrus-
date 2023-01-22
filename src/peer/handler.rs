use crate::message::Message;
use crate::peer::PeerSession;
use crate::piece::PieceHandler;
use crate::{PeerId, Receiver, Result, Sender};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::{net::TcpStream, sync::mpsc::unbounded_channel};

/// Handles all Peer events
pub struct PeerHandler {
    /// Keeps track of all the pieces in the torrent
    piece_handler: Arc<RwLock<PieceHandler>>,
    /// The peer sends messages through sender
    sender: Sender,
    /// Event Receiver
    receiver: Receiver,
    /// Torrent Peers
    peers: HashMap<PeerId, Sender>,
}

impl PeerHandler {
    pub fn new(piece_handler: Arc<RwLock<PieceHandler>>) -> Self {
        let peers = HashMap::new();

        let (sender, receiver) = unbounded_channel();
        Self {
            receiver,
            piece_handler,
            peers,
            sender,
        }
    }

    pub fn insert_peers(&mut self, peer_id: PeerId, stream: TcpStream) {
        let (sender, mut peer_session) = PeerSession::new(self.sender.clone());

        tokio::spawn(async move {
            if let Err(_) = peer_session.start(stream).await {
                return;
            }
        });

        self.peers.insert(peer_id, sender);
    }

    pub async fn start_handling(mut self) -> Result<()> {
        loop {
            if let Some(msg) = self.receiver.recv().await {
                if let Err(err) = self.handle_message(msg) {
                    log::error!("Error : {}",err);
                    continue;
                }
            }
        }
    }

    fn handle_message(&mut self, msg: Message) -> Result<()> {
        use Message::*;

        match msg {
            Piece(block) => {
                while self.piece_handler.try_write().is_err() {}

                Ok(self.piece_handler.write().unwrap().insert_block(block)?)
            }
            _ => {
                unimplemented!("Handle all swarm message types");
            }
        }
    }
}
