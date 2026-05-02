use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::TcpStream};

use crate::game::{ClientMessage, ServerMessage};

pub async fn recv_client_message(read: &mut ReadHalf<TcpStream>) -> Result<ClientMessage, postcard::Error> {
    let length = read.read_u32().await.unwrap();

    let mut buf = vec![0u8; length as usize];
    read.read_exact(&mut buf).await.unwrap();
    let decoded = postcard::from_bytes(&buf);

    decoded
}

pub async fn send_server_message(write: &mut WriteHalf<TcpStream>, msg: &ServerMessage) -> Result<(), postcard::Error> {
    let encoded = postcard::to_allocvec(msg)?;
    let length = encoded.len() as u32;
    
    write.write_u32(length).await.unwrap();
    write.write_all(&encoded).await.unwrap();

    Ok(())
}