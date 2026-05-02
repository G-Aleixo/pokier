use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};

use crate::game::{ClientMessage, ServerMessage};

pub async fn recv_client_message<R>(read: &mut R) -> Result<ClientMessage, postcard::Error>
    where R: AsyncRead + Unpin
{
    let length = read.read_u32().await.unwrap();

    let mut buf = vec![0u8; length as usize];
    read.read_exact(&mut buf).await.unwrap();
    postcard::from_bytes(&buf)
}

pub async fn send_server_message<W>(write: &mut W, msg: &ServerMessage) -> Result<(), postcard::Error>
    where W: AsyncWrite + Unpin
{
    let encoded = postcard::to_allocvec(msg)?;
    let length = encoded.len() as u32;
    
    write.write_u32(length).await.unwrap();
    write.write_all(&encoded).await.unwrap();

    Ok(())
}