use crate::encode::{Encode, Decode};

use crate::card;

// break recursivity of the expected field
#[derive(Encode, Decode, PartialEq, Eq)]
#[derive(Debug)]
pub enum ExpectedKind {
    GetName,
    Name,
    Turn,
    Bet,
    Fold,
    Check,
    Skip,
    Ready,
    Start,
    Deal,
}

#[derive(Encode, Decode, PartialEq, Eq)]
#[derive(Debug)]
pub enum PacketError {
    UnexpectedPacket {expected: ExpectedKind},
    MalformedData,
}

#[derive(Encode, Decode, PartialEq, Eq)]
#[derive(Debug)]
pub enum Packet {
    GetName,
    Name {name: String},
    Turn,
    Bet {amount: u32},
    Fold,
    Check,
    Skip,
    InvalidPacket {error: PacketError},
    Ready,
    Start,
    Deal {cards: Vec<card::Card>},
}

#[cfg(test)]
mod tests {
    use crate::encode::{decode, encode};

    use super::*;

    #[test]
    fn works_bet() {
        let packet = Packet::Bet { amount: 542 };
        let encoded = encode(&packet);
        let decoded: Packet = decode(&encoded).unwrap();

        println!("{encoded:?}");
        assert_eq!(packet, decoded);
    }
}