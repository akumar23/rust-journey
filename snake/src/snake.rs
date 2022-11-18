use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color=[0.00, 0.80, 0.00, 1.0];

pub enum Direction {
    Up,
    Down,
    Left, 
    Right,
}

