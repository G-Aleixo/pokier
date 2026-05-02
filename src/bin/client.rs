use pokier::net::client::{recv_server_message, send_client_message};
use tokio::net::{TcpStream};

#[tokio::main]
async fn main() {
    let mut conn = TcpStream::connect("127.0.0.1:7773").await.unwrap();
    
    println!("sending join");
    send_client_message(&mut conn, &pokier::game::ClientMessage::PlayerJoin {}).await.unwrap();

    println!("sending request");
    send_client_message(&mut conn, &pokier::game::ClientMessage::RequestSnapshot {}).await.unwrap();
    loop {
        println!("awaiting msg");
        println!("{:#?}", recv_server_message(&mut conn).await.unwrap());
    }
}