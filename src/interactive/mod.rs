use std::io::{BufRead, Read};

use algos::algo_move;

use crate::algos;
use crate::chess::{Coord, Game, GameState, Move};

pub struct InteractiveMode {
    game: Game,
}

impl InteractiveMode {
    pub fn new() -> Self {
        Self {
            game: Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap(),
        }
    }

    pub fn log_state(&self, state: GameState) {
        match state {
            GameState::Check(c) => {
                println!("INFO {} is in check", c)
            }
            GameState::Checkmate(c) => {
                println!("INFO game ended. checkmate for {}", c)
            }
            GameState::Stalemate => {
                println!("INFO game ended. stalemate")
            }
            GameState::Draw => {
                println!("INFO game ended. draw.")
            }
            GameState::Normal => {}
        }
    }

    pub fn launch(&mut self) {
        let stdin = std::io::stdin();
        for line_r in stdin.lock().lines() {
            let line = line_r.expect("could not read stdin");
            let spl = line.split(" ").collect::<Vec<_>>();
            match spl[0] {
                "load" => match Game::from_fen(spl[1..].join(" ").as_str()) {
                    Err(msg) => println!("ERROR: {}", msg),
                    Ok(game) => {
                        self.game = game;
                        println!("OK");
                    }
                },
                "dump" => {
                    println!("OK {}", self.game.to_fen())
                }
                "dump_debug" => {
                    println!("{}\nhttps://lichess.org/editor/{}\nOK",self.game,self.game.to_fen().replace(" ", "_"))
                }
                "possible_moves" => {
                    if spl.len() != 2 {
                        println!("ERROR: argument count is incorrect")
                    } else {
                        match Coord::from_algebraic(spl[1]) {
                            Err(msg) => println!("ERROR: {}", msg),
                            Ok(c) => {
                                let moves = self.game.get_possible_moves(&c);
                                println!("OK");
                                for m in moves {
                                    println!("{}", m.serialize());
                                }
                            }
                        }
                    }
                }
                "all_possible_moves" => {
                    let moves = self.game.get_all_possible_moves();
                    println!("OK");
                    for m in moves {
                        println!("{}", m.serialize());
                    }
                }
                "move" => {
                    if spl.len() != 2 {
                        println!("ERROR: argument count is incorrect")
                    } else {
                        let m = Move::deserialize(spl[1]);
                        match self.game.make_move(&m) {
                            Err(msg) => {
                                println!("WARN while applying move: {}", msg)
                            }
                            Ok(state) => self.log_state(state),
                        }
                    }
                }
                "algo" => {
                    if spl.len() != 3 {
                        println!("ERROR: argument count is incorrect")
                    } else {
                        match algo_move(spl[1], &self.game) {
                            Err(msg) => println!("ERROR: {}", msg),
                            Ok(m) => {
                                match spl[2] {
                                    "false" => {}
                                    "true" => match self.game.make_move(&m) {
                                        Err(msg) => {
                                            println!("WARN while applying move: {}", msg)
                                        }
                                        Ok(state) => self.log_state(state),
                                    },
                                    _ => {
                                        println!("WARN 'do move' argument invalid")
                                    }
                                }
                                println!("OK {}", m)
                            }
                        }
                    }
                }
                "quit" => {
                    break;
                }
                _ => println!("ERROR: command not found!"),
            }
        }
    }
}
