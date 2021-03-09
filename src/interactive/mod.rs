use std::io::{BufRead, Read};

use crate::chess::{Coord, Game};

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
                "possible_moves" => {
                    if spl.len() < 2 {
                        println!("ERROR: no argument")
                    } else {
                        match Coord::from_algebraic(spl[1]) {
                            Err(msg) => return println!("ERROR: {}", msg),
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
                "move" => {
                    todo!()
                }
                "move_algo" => {
                    todo!()
                }
                "quit" => {
                    break;
                }
                _ => println!("ERROR: command not found!"),
            }
        }
    }
}
