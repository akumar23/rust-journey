extern crate args;
extern crate getopts;

use chess:: {Board, MoveGen, Color, BoardStatus, ChessMove, ALL_RANKS, Piece, get_rank };
use std::env;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};
use args::{Args, ArgsError};
use getopts::Occur;
use colored::{ColoredString, Colorize};
mod piece_values;
mod benchmarks;


fn main() {
    println!("Hello, world!");
}
