use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};

use crate::game::{ClientMessage, ServerMessage};

pub async fn recv_server_message<R>(read: &mut R) -> std::io::Result<ServerMessage>
    where R: AsyncRead + Unpin
{
    let length = read.read_u32().await?;

    let mut buf = vec![0u8; length as usize];

    read.read_exact(&mut buf).await?;
    Ok(postcard::from_bytes(&buf).unwrap_or_else(|_| panic!("postcard failed to deserialize data: {:#?}", &buf)))
}

pub async fn send_client_message<W>(write: &mut W, msg: &ClientMessage) -> std::io::Result<()>
    where W: AsyncWrite + Unpin
{
    let encoded = postcard::to_allocvec(msg).expect("postcard failed to serialize data");
    let length = encoded.len() as u32;
    
    write.write_u32(length).await?;
    write.write_all(&encoded).await?;

    Ok(())
}