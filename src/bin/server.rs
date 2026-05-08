use std::{collections::HashMap, sync::Arc};

use pokier::game::{ClientMessage, GameServer, GameState, ServerMessage};
use tokio::{io::{ReadHalf, WriteHalf, split}, net::{TcpListener, TcpStream}, sync::{Mutex, mpsc}};
use uuid::{Uuid};

#[tokio::main]
async fn main() {
    let outgoing = Arc::new(Mutex::new(HashMap::new()));
    
    let (tx, rx) = mpsc::channel(16);
    let mut server = GameServer {
        state: GameState::default(),
        incoming: rx,
        outgoing: outgoing.clone()
    };
    
    tokio::spawn(listen(outgoing.clone(), tx));

    server.run().await;
    println!("hello world!");
}

async fn listen(outgoing: Arc<Mutex<HashMap<Uuid, mpsc::Sender<ServerMessage>>>>, tx: mpsc::Sender<(ClientMessage, Uuid)>) {
    let listener = TcpListener::bind("127.0.0.1:7773").await.unwrap();

    loop {
        let (client, addr) = listener.accept().await.unwrap();
        println!("client at {addr} connected");

        let uuid = uuid::Uuid::new_v4();
        let (read, write) = split(client);
    
        tokio::spawn(read_handler(read, tx.clone(), uuid));

        let (tx, rx) = mpsc::channel(16);

        outgoing.lock().await.insert(uuid, tx);

        tokio::spawn(write_handler(write, rx));
    }
}

async fn read_handler(mut reader: ReadHalf<TcpStream>, tx: mpsc::Sender<(ClientMessage, Uuid)>, uuid: Uuid) {
    while let Ok(msg) = pokier::net::server::recv_client_message(&mut reader).await {
        tx.send((msg, uuid)).await.unwrap();
    }
}

async fn write_handler(mut writer: WriteHalf<TcpStream>, mut rx: mpsc::Receiver<ServerMessage>) -> std::io::Result<()> {
    while let Some(msg) = rx.recv().await {
        pokier::net::server::send_server_message(&mut writer, &msg).await?;
    }

    Ok(())
}