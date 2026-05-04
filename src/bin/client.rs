use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use pokier::{game::{ClientMessage, GameStateSnapshot, ServerMessage}, net::client::{recv_server_message, send_client_message}};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}};
use tokio::{io::{AsyncRead, AsyncWrite, split}, net::TcpStream, sync::mpsc};


#[tokio::main]
async fn main() -> std::io::Result<()>{
    let mut terminal = ratatui::init();

    let (recv, send) = split(TcpStream::connect("127.0.0.1:7773").await.unwrap());

    let (tx, rx) = mpsc::channel(128);
    let crossterm_tx = tx.clone();

    tokio::spawn(async { recv_message(tx, recv).await; });
    tokio::spawn(async { read_crossterm_event(crossterm_tx).await; });

    let result = App::new(rx, send).run(&mut terminal).await;

    ratatui::restore();

    result
}

// sending messages is handled directly in the code
// maybe will change to move it to a specific task to handle it
async fn recv_message<R>(tx: mpsc::Sender<ClientEvent>, mut reader: R)
where R: AsyncRead + Unpin
{
    loop {
        let message = recv_server_message(&mut reader).await.unwrap();
    
        tx.send(ClientEvent::ServerMessage(message)).await.unwrap();
    }
}

async fn read_crossterm_event(tx: mpsc::Sender<ClientEvent>) {
    loop {
        match event::read().unwrap() {
            Event::Key(key_event) => tx.send(ClientEvent::Input(key_event)).await.unwrap(),
            _ => {}
        }
    }
}

struct App<W>
where W: AsyncWrite + Unpin
{
    game_state: Option<GameStateSnapshot>,
    event_rx: mpsc::Receiver<ClientEvent>,
    server_write: W,
    exit: bool,
}

impl<W: AsyncWrite + Unpin> App<W> {
    async fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        // send join message to the pokier server
        send_client_message(&mut self.server_write, &ClientMessage::PlayerJoin).await.unwrap();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            
            self.handle_events().await?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    async fn handle_events(&mut self) -> std::io::Result<()> {
        match self.event_rx.recv().await.unwrap() {
            ClientEvent::Input(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            ClientEvent::ServerMessage(message) => {
                match message {
                    ServerMessage::Snapshot(state) => {
                        self.game_state = Some(state);
                    }
                }
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => { self.exit = true }
            KeyCode::Char('p') => { println!("{:#?}", self.game_state) }
            _ => {}
        }
    }
}

enum ClientEvent {
    Input(KeyEvent),
    ServerMessage(ServerMessage)
}

impl<W: AsyncWrite + Unpin> App<W> {
    fn new(event_rx: mpsc::Receiver<ClientEvent>, server_write: W) -> Self {
        App {
            event_rx: event_rx,
            server_write: server_write,
            game_state: None,
            exit: false,
        }
    }
}

impl<W: AsyncWrite + Unpin> Widget for &App<W> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where Self: Sized {
        let block = Block::bordered().title(Line::from(" Testing the title :) ").centered()).title_bottom(Line::from(vec![" Quit ".into(), "<Q>".blue().bold(), " ".into()]).centered()).border_set(border::THICK);

        Paragraph::new(Text::from("Triying out the text")).centered().block(block).render(area, buf);
    }
}