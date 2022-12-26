use tokio::io::Error;
use tokio_util::codec::{Decoder, Encoder};

pub struct HandShake {}

pub struct HandShakeCodec;

/// All messages except the 'Handshake' message as described in Wire Protocol
pub enum Message {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
    Port,
}

pub struct PeerCodec;

impl Encoder<Message> for PeerCodec {
    type Error = Error;

    fn encode(&mut self, item: Message, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        todo!();
    }
}

impl Decoder for PeerCodec {
    type Error = Error;
    type Item = Message;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
