use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};

use crate::game::{ClientMessage, ServerMessage};

pub async fn recv_server_message<R>(read: &mut R) -> Result<ServerMessage, postcard::Error>
    where R: AsyncRead + Unpin
{
    let length = read.read_u32().await.unwrap();

    let mut buf = vec![0u8; length as usize];
    read.read_exact(&mut buf).await.unwrap();
    let decoded = postcard::from_bytes(&buf);

    decoded
}

pub async fn send_client_message<W>(write: &mut W, msg: &ClientMessage) -> Result<(), postcard::Error>
    where W: AsyncWrite + Unpin
{
    let encoded = postcard::to_allocvec(msg)?;
    let length = encoded.len() as u32;
    
    write.write_u32(length).await.unwrap();
    write.write_all(&encoded).await.unwrap();

    Ok(())
}