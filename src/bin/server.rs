use std::collections::HashMap;

use tokio::{
    net::{
        TcpListener, TcpStream,
    },
    sync::mpsc, task
};
use pokier::{
    player
};

// transmited between client and game handlers within a tuple with origin
enum GameMessage {
    NewHandler(player::Player),
}

struct ClientHandler {
    task_id: task::Id,
    // send data to
    tx: mpsc::Sender<GameMessage>,
    // receiving data will be handled by a global mpsc
}

async fn handle_client(_stream: TcpStream, tasks_tx: mpsc::Sender<ClientHandler>) {
    // send message to game handler that a player joined
    tasks_tx.send(GameMessage::NewHandler)
}

async fn game_handler(tasks_rx: mpsc::Receiver<ClientHandler>, player_count_tx: mpsc::Sender<bool>) {
    // map handle client task id to it's info
    let mut handlers: HashMap<task::Id, ClientHandler> = HashMap::new();
}

#[tokio::main]
async fn main() {
    let config = pokier::config::get_config();

    let listener = TcpListener::bind(config.server.ip + ":" + config.server.port.to_string().as_str()).await.unwrap();

    let (tasks_tx, tasks_rx) = mpsc::channel(32);

    let (player_count_tx , mut player_count_rx) = mpsc::channel::<bool>(4);

    tokio::task::spawn(game_handler(tasks_rx, player_count_tx));

    while !player_count_rx.recv().await.unwrap() {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::task::spawn(async move {
            handle_client(stream);
        });
    };
}