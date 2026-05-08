use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use pokier::{game::{ClientMessage, GameStateSnapshot, PlayerStatus, ServerMessage}, net::client::{recv_server_message, send_client_message}};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Direction, Layout, Rect, Size}, style::Stylize, symbols::{border, merge::MergeStrategy}, text::{Line, Text}, widgets::{Block, Paragraph, Widget}};
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
        if let Ok(message) = recv_server_message(&mut reader).await {
            tx.send(ClientEvent::ServerMessage(message)).await.unwrap();
        }
    }
}

async fn read_crossterm_event(tx: mpsc::Sender<ClientEvent>) {
    loop {
        match event::read().unwrap() {
            Event::Key(key_event) => {
                tx.send(ClientEvent::Input(key_event)).await.unwrap();
            
                if key_event.code == KeyCode::Char('q') {
                    break
                }

            }
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
            event_rx,
            server_write,
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

        match &self.game_state {
            None => {
                let msg = String::from("\nServer not connected");
                let area = area.centered(Constraint::Length(msg.len() as u16 + 5), Constraint::Length(5));
                
                let block = Block::bordered().border_set(border::THICK);
                
                Paragraph::new(Text::from(msg)).centered().block(block).render(area, buf);
            }

            Some(state) => {
                let state = state.clone();

                let horizontal_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(70),
                        Constraint::Min(22),
                    ])
                    .split(area);

                let player_list_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(40),
                        Constraint::Percentage(60)
                    ])
                    .split(horizontal_layout[1]);
                let round_order = player_list_layout[0];
                let round_order = round_order.resize(Size::new(round_order.width, round_order.height + 1));
                let other_players = player_list_layout[1];

                for (i, id) in state.turn_order.iter().enumerate() {
                    let area = Rect::new(round_order.x + 1, round_order.y + 1 + i as u16, round_order.width - 2, 1);

                    let player = state.players.get(id).unwrap();

                    let mut text = Text::from(format!("{:10}: {}", player.id, player.score));

                    if matches!(player.status, PlayerStatus::Disconnected | PlayerStatus::Folded) {
                        text = text.gray();
                    }

                    text.render(area, buf);
                }

                Block::bordered().border_set(border::THICK).merge_borders(MergeStrategy::Exact).render(round_order, buf);

                for (i, player_state) in state.players.values().enumerate() {
                    let area = Rect::new(other_players.x + 1,other_players.y + 1 + i as u16,other_players.width - 2, 1);
                
                    let mut text = Text::from(format!("{:10}: {}", player_state.id, player_state.score));

                    if matches!(player_state.status, PlayerStatus::Disconnected | PlayerStatus::Waiting) {
                        text = text.gray();
                    }

                    if player_state.id == state.my_id {
                        text = text.light_green();
                    }

                    text.render(area, buf);
                };

                Block::bordered().border_set(border::THICK).merge_borders(MergeStrategy::Exact).render(other_players, buf);
            }
        }

        block.merge_borders(MergeStrategy::Fuzzy).render(area, buf);
    }
}



//TODO: implement the left and main side of ui
// + connection interface
// + lan discovery