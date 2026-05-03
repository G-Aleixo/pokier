use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, mpsc};

use crate::game::{ClientAction, Deck, GameStateSnapshot, GameStatus, PlayerState, PlayerStatus, evaluate_hand, message::{ClientMessage, ServerMessage}, state::{GameState, PlayerId}};

#[derive(Debug)]
pub struct GameServer {
    pub state: GameState,
    pub incoming: mpsc::Receiver<(ClientMessage, PlayerId)>,
    pub outgoing: Arc<Mutex<HashMap<PlayerId, mpsc::Sender<ServerMessage>>>>,
}

impl GameServer {
    /// runs forever until there are no more senders
    pub async fn run(&mut self) {
        while let Some((msg, from)) = self.incoming.recv().await {
            self.handle_message(msg, from).await;

            if self.state.status == GameStatus::GameEnd {
                self.reset_state();
            }

            // check if it's time to show another card
            if !matches!(self.state.status, GameStatus::GameEnd | GameStatus::WaitingPlayers) {
                if self.state.current_turn >= self.state.turn_order.len() {
                    self.state.current_turn %= self.state.turn_order.len();
                }
                
                // advance current_turn until a player is able to play
                let starting = self.state.current_turn;
                while !self.can_play(&self.state.turn_order[self.state.current_turn]) {
                    self.state.current_turn += 1;

                    self.state.current_turn %= self.state.turn_order.len();

                    if self.state.current_turn == starting {
                        self.state.resolved = true;
                        break
                    }

                    self.dirty();
                }

                // advance to next round
                if self.state.resolved {
                    if self.state.status == GameStatus::FinalBet {
                        let winners = self.determine_winner();

                        // split the pot among everyone
                        let per_player = self.state.pool.checked_div(winners.len() as u32).unwrap_or(0);

                        for id in winners {
                            self.state.players.get_mut(&id).unwrap().score += per_player;
                        }

                        self.state.status = GameStatus::GameEnd;
                        break;
                    }

                    self.deal_public();
                    self.state.current_turn = 0;

                    self.dirty();   
                }
            }
        
            if self.state.dirty {
                self.broadcast_state().await;
                self.state.dirty = false;
            }
        }
    }

    pub fn is_turn(&self, player: &PlayerId) -> bool {
        if let Some(player_state) = self.state.players.get(player) && self.state.turn_order[self.state.current_turn] == player_state.id {
            return true
        }
        false
    }

    pub fn can_play(&self, player: &PlayerId) -> bool {
        self.is_turn(player) && self.state.players.get(player).unwrap().status == PlayerStatus::Playing
    }

    pub fn dirty(&mut self) {
        self.state.dirty = true;
    }

    pub async fn handle_message(&mut self, msg: ClientMessage, from: PlayerId) {
        println!("{msg:#?}");
        match msg {
            ClientMessage::RequestSnapshot => {
                self.send_to(
                &ServerMessage::Snapshot(
                    GameStateSnapshot::from_game_state(&self.state, &from)),
                    &from
                ).await.unwrap();
            }
            ClientMessage::PlayerJoin => {
                // add the new player to the state
                println!("inserted player at {from}");
                self.state.players.insert(from, PlayerState {
                    id: from,
                    hand: vec![],
                    status: PlayerStatus::Waiting,
                    score: 100,
                    betted: 0,
                });
                // check for connected player amount and auto start
                if self.connected_count() >= 3 && self.state.status == GameStatus::WaitingPlayers {
                    self.state.status = GameStatus::InRound;

                    self.setup_game().await;
                }

                self.dirty();
            }
            ClientMessage::Action(action) => {
                if matches!(self.state.status, GameStatus::GameEnd | GameStatus::WaitingPlayers) {
                    return
                }
                if self.is_turn(&from) {
                    match action {
                        ClientAction::Bet(amount) => {
                            let player = self.state.players.get_mut(&from).unwrap();
                            // includes 0 bets as a kind of all-in
                            if (amount <= player.score
                                && player.betted + amount >= self.state.min_bet)
                                || player.score - amount == 0
                             {
                                
                                player.score -= amount;
                                player.betted += amount;
                                
                                self.state.min_bet = self.state.min_bet.max(player.betted);
                                
                                self.state.pool += amount;
                                
                                self.state.current_turn += 1;
                                self.dirty();
                            }
                        }
                        ClientAction::Fold => {
                            self.state.players.get_mut(&from).unwrap().status = PlayerStatus::Folded;

                            self.state.current_turn += 1;
                            self.dirty();
                        }
                    }
                }
            }
        }
    }

    pub async fn broadcast(&mut self, msg: &ServerMessage) {
        let mut failed = vec![];

        for (player_id, tx) in &(*self.outgoing.lock().await) {
            if let Err(e) = tx.send(msg.clone()).await {
                eprintln!("Failed to send to player id {player_id}: {e}");

                failed.push(*player_id);
            }
        }

        for player_id in failed {
            self.mark_disconnected(player_id);
        }
    }

    async fn send_to(&self, msg: &ServerMessage, player: &PlayerId) -> Result<(), mpsc::error::SendError<ServerMessage>> {
        if let Some(tx) = self.outgoing.lock().await.get(player) {
            tx.send(msg.clone()).await?;
            println!("sent to tx {player}");
        }
        Ok(())
    }

    pub async fn send_to_tx(&self, msg: &ServerMessage, tx: &mpsc::Sender<ServerMessage>) -> Result<(), mpsc::error::SendError<ServerMessage>> {
        tx.send(msg.clone()).await
    }

    async fn broadcast_state(&mut self) {
        // has to be different as message changes depending on the player
        let mut failed = vec![];
        
        {
            let outgoing = self.outgoing.lock().await;
            for (player_id, tx) in outgoing.iter() {
                let state = GameStateSnapshot::from_game_state(&self.state, player_id);
                if let Err(e) = self.send_to_tx(&ServerMessage::Snapshot(state), tx).await {
                    eprintln!("Failed to send to player id {player_id}: {e}");
                    failed.push(*player_id);
                };
                println!("sent to player {player_id}");
            };
        }

        for id in failed {
            self.mark_disconnected(id);
        }
    }

    fn mark_disconnected(&mut self, player_id: PlayerId) {
        if let Some(player) = self.state.players.get_mut(&player_id) {
            player.status = PlayerStatus::Disconnected;
        };
    }

    fn connected_count(&self) -> usize {
        self.state.players.values().filter(|player| {
            matches!(player.status,
                PlayerStatus::Waiting |
                PlayerStatus::Playing |
                PlayerStatus::Folded
            )
        }).count()
    }

    fn reset_state(&mut self) {
        self.state.deck = Deck::new().shuffled();
        self.state.discarded = vec![];
        self.state.river = vec![];
        self.state.current_turn = 0;

        self.state.status = GameStatus::WaitingPlayers;

        for player in self.state.players.values_mut() {
            player.betted = 0;
            player.hand = vec![]
        }

        self.dirty();
    }

    pub async fn new_game(&mut self) {
        // reset deck and stuff
        self.reset_state();

        // deal out the hands
        for player in self.state.players.values_mut() {
            player.hand.append(&mut self.state.deck.deal(2));
        }

        self.broadcast_state().await;
    }

    async fn setup_game(&mut self) {
        // pull all the players into the turn order and deal cards once
        for (player_id, player_state) in self.state.players.iter_mut().filter(|(_, player_state)| {player_state.status == PlayerStatus::Waiting}) {
            player_state.status = PlayerStatus::Playing;
            self.state.turn_order.push(*player_id);
            
            // deal the 2 cards
            player_state.hand.append(&mut self.state.deck.deal(2));
        }

        self.dirty();
    }

    fn deal_public(&mut self) {
        // check for the amount of card already on the table to deal more
        match self.state.river.len() {
            0 => {
                self.state.river.append(&mut self.state.deck.deal(3));
            }
            3 => {
                self.state.river.push(self.state.deck.deal_one().unwrap());
            }
            4 => {
                self.state.river.push(self.state.deck.deal_one().unwrap());
                self.state.status = GameStatus::FinalBet;
            }
            5 => {
                // do nothing
            }
            _ => {
                panic!("Impossible amount of cards in river {}", self.state.river.len());
            }
        };

        self.dirty();
    }

    fn determine_winner(&self) -> Vec<PlayerId> {
        let hands: Vec<_> = self.state.players.iter().filter(|(id, _)| self.can_play(id)).map(|(id, player_state)| {(*id, [player_state.hand.clone(), self.state.river.clone()].concat())}).collect();
        let strengths: Vec<_> = hands.iter().map(|(from, hand)| (from, evaluate_hand(hand))).collect();

        let best = match strengths.iter().max() {
            Some(v) => v,
            None => {
                return vec![]
            }
        };

        let aaaa: Vec<_> = strengths.iter().filter(|(_, s)| *s == best.1).map(|(i, _)| *i).collect();
    
        aaaa.iter().map(|v| **v).collect()
    }
}