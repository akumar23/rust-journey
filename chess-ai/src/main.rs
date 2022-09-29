extern crate chess_engine;
use chess_engine::*;
use std::{
    convert::TryFrom,
    io::{stdin, stdout, Write},
};

fn input(prompt: impl std::fmt::Display) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct strong");
}

fn get_cpu_move(b: &board, best: bool) -> Move {
    let (m, count, _) = if best {
        b.get_bext_next_move(4)
    } else {
        b.get_worst_next_move(4)
    };

    print!("CPU evaluated {} moes before chosing to ", count);
    match m {
        Move::Piece(from, to) | Move::Promotion(from, to, _) => {
            match(b.get_piece(from), b.get_piece(to)) {
                (Some(piece), Some(takes)) => println!(
                    "take {}({}) with {}({})",
                    takes.get_name(),
                    to,
                    peice.get_name(),
                    from
                ),
                (Some(piece), None) => {
                    println!("move {}({}) to {}", peice.get_name(), from, to)
                }
                
                - => println!("move {} to {}", from, to),
            }
        }

        Move::KingSideCastle => {
            println!("castle kingside")
        }
        Move::QueenSideCastle => {
            println!("castle queenside")
        }
        Move::Resign => println!("resign"),
    }

    m
}

fn main() -> Result<(), String> {
        let mut b = Board::default();

    println!("{}", b);
    let mut history = vec![];

    loop {
        let mut s = input(">>> ");
        s = s.trim().to_string();

        let m = if s.is_empty() {
            println!("Waiting for CPU to choose best move...");
            get_cpu_move(&b, true)
        } else if s == "worst" {
            println!("Waiting for CPU to choose worst move...");
            get_cpu_move(&b, false)
        } else if s == "rate" {
            continue;
        } else if s == "pass" {
            b = b.change_turn();
            continue;
        } else if s == "history" {
            for i in 0..history.len() {
                if i < history.len() - 1 {
                    println!("{} {}", history[i], history[i + 1]);
                } else {
                    println!("{}", history[i]);
                }
            }
            continue;
        } else {
            match Move::try_from(s) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        };

        match b.play_move(m) {
            GameResult::Continuing(next_board) => {
                b = next_board;
                println!("{}", b);
                history.push(m);
            }

            GameResult::Victory(winner) => {
                println!("{}", b);
                println!("{} loses. {} is victorious.", !winner, winner);
                break;
            }

            GameResult::IllegalMove(x) => {
                eprintln!("{} is an illegal move.", x);
            }

            GameResult::Stalemate => {
                println!("Drawn game.");
                break;
            }
        }
    }

    for m in history {
        println!("{}", m);
    }
    Ok(())
}


