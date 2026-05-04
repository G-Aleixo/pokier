use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use pokier::{game::{ClientMessage, GameStateSnapshot, ServerMessage}, net::client::{recv_server_message, send_client_message}};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Direction, Flex, Layout, Rect}, macros::constraints, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}};
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
            Event::Resize(width, height) => tx.send(ClientEvent::Resize(width, height)).await.unwrap(),
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
    window_size: Rect,
    exit: bool,
}

impl<W: AsyncWrite + Unpin> App<W> {
    async fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        // send join message to the pokier server
        send_client_message(&mut self.server_write, &ClientMessage::PlayerJoin).await.unwrap();

        while !self.exit {
            terminal.resize(self.window_size)?;
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
            ClientEvent::Resize(width, height) => {
                self.window_size = Rect::new(0, 0, width, height);
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
    Resize(u16, u16),
    ServerMessage(ServerMessage)
}

impl<W: AsyncWrite + Unpin> App<W> {
    fn new(event_rx: mpsc::Receiver<ClientEvent>, server_write: W) -> Self {
        App {
            event_rx: event_rx,
            server_write: server_write,
            game_state: None,
            window_size: Rect::default(),
            exit: false,
        }
    }
}

impl<W: AsyncWrite + Unpin> Widget for &App<W> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where Self: Sized {
        let title = Line::from(" Pokier TUI Client ");
        let instructions = Line::from(vec![" Quit ".into(), "<Q>".blue().bold(), " ".into()]);

        let block = Block::bordered().title(title.centered()).title_bottom(instructions.centered()).border_set(border::THICK);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(70),
                Constraint::Min(22),
            ])
            .split(area);

        self.game_state.clone().and_then(|state| {
            let player_list: Vec<_> =  state.players;
            for i in 0..player_list.len() {
                let area = Rect::new(horizontal_layout[1].x + 1, horizontal_layout[1].y + 1 + i as u16, horizontal_layout[1].width - 2, 1);
            
                Text::from(format!("{:10}: {}", player_list[i].id, player_list[i].score.to_string())).render(area, buf);
            };

            Some(())
        });

        Paragraph::new(Text::from("Trying out the text")).centered().block(block).render(horizontal_layout[0], buf);
    }
}