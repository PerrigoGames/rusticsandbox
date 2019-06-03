use piston_window::types::Color;

use crate::draw::draw_block;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct Player {
    dir: Direction,
    body: LinkedList<Block>,
}

impl Player {
    
}