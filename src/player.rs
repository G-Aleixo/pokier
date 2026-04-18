use std::{
    io::prelude::*,
    net::TcpStream
};
use crate::{
    protocol::packet::{
        Packet,
        PacketError,
        ExpectedKind,
    },
    card,
    encode::{
        encode,
        decode,
    }
};

pub enum PlayerState {
    PreMatch,
    Idle,
    Ready,
    WaitingForAction, // action may be Bet, Fold or Stand/Check
}

pub struct Player {
    pub stream: TcpStream,
    pub name: String,
    pub state: PlayerState,
    pub hand: Option<[card::Card; 2]>,
    pub chips: u32,
    pub bet_amount: u32,
    pub folded: bool,
}

impl Player {
    pub fn new(stream: TcpStream, name: String, state: PlayerState, chips: u32) -> Self {
        Self {
            stream,
            name,
            state,
            chips,
            hand: None,
            bet_amount: 0,
            folded: false,
        }
    }
    
    pub fn from_stream(mut stream: TcpStream, default_chips: Option<u32>) -> Option<Self> {
        // initialize a Player from a socket connection
        stream.write(&encode(&Packet::GetName)).unwrap();
    
        let mut name = Vec::new();
        stream.read_to_end(&mut name).unwrap();

        if let Packet::Name { name } = decode(&name).unwrap() {
            return Some(Self {
                stream,
                name,
                state: PlayerState::PreMatch,
                chips: default_chips.unwrap_or(100),
                hand: None,
                bet_amount: 0,
                folded: false,
            });
        } else {
            stream.write(&encode(
                &Packet::InvalidPacket { error: PacketError::UnexpectedPacket { expected: ExpectedKind::Name } }
            )).unwrap();
            return None;
        };
    }
}