use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
    color: Color,
}

impl Snake {
    pub fn new(x: i32, y: i32, color: Color) -> Snake {
        let mut body = LinkedList::<Block>::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
            color,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(self.color, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let block = self.body.front().unwrap();
        (block.x, block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let moving_dir = match dir {
            Some(d) => d,
            None => self.direction,
        };

        let (head_x, head_y) = self.head_position();

        match moving_dir {
            Direction::Down => (head_x, head_y + 1),
            Direction::Up => (head_x, head_y - 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }
        let (new_x, new_y) = self.next_head(None);
        self.body.push_front(Block { x: new_x, y: new_y });
        self.tail = self.body.pop_back();
    }

    pub fn restore_tail(&mut self) {
        let tail_block = self.tail.clone().unwrap();
        self.body.push_back(tail_block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        // let b = Block { x, y };
        for (pos, block) in self.body.iter().enumerate() {
            // if element is tail, then no real overlap
            if pos == self.body.len() {
                break;
            }
            if block.x == x && block.y == y {
                return true;
            }
        }
        return false;
    }
}
