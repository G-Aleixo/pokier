use std::{
    io::prelude::*,
};


pub mod packet;

#[allow(non_camel_case_types)]
type uLengthPrefix = u16;

pub fn send_packet<R: Write>(stream: &mut R, packet: &packet::Packet) -> Result<(), std::io::Error> {
    let encoded = crate::encode::encode(packet);
    let size = encoded.len() as uLengthPrefix;

    stream.write_all(&size.to_be_bytes())?;
    stream.write_all(&encoded)?;

    Ok(())
}

pub fn recv_packet<R: Read>(stream: &mut R) -> Result<packet::Packet, std::io::Error> {
    let mut buf = [0; (uLengthPrefix::BITS / 8) as usize];
    stream.read_exact(&mut buf)?;
    let size = uLengthPrefix::from_be_bytes(buf);
    
    let mut buf = vec![0; size as usize];
    stream.read_exact(&mut buf)?;

    let packet = crate::encode::decode(&buf).unwrap();

    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_recv() -> Result<(), std::io::Error> {
        let mut buf = vec![0; 0];
        
        let packet = packet::Packet::Bet { amount: 4423 };

        send_packet(&mut buf, &packet)?;

        let mut cursor = std::io::Cursor::new(buf);

        assert_eq!(recv_packet(&mut cursor).unwrap(), packet);
        Ok(())
    }
}