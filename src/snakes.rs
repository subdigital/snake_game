use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0., 0.8, 0., 1.];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    pub direction: Direction,
    pub next_direction: Option<Direction>,
    pub speed: f64, // in blocks_per_second
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            next_direction: None,
            speed: 8.0,
            body,
            tail: None,
        }
    }

    pub fn turn_left(&mut self) {
        self.next_direction = Some(Direction::Left);
    }

    pub fn turn_right(&mut self) {
        self.next_direction = Some(Direction::Right);
    }

    pub fn turn_up(&mut self) {
        self.next_direction = Some(Direction::Up);
    }

    pub fn turn_down(&mut self) {
        self.next_direction = Some(Direction::Down);
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, context, graphics);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self) {
        if let Some(next) = self.next_direction {
            if next != self.direction.opposite() {
                self.direction = next;
            }
        }

        let (x, y) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block { x, y: y - 1 },
            Direction::Down => Block { x, y: y + 1 },
            Direction::Left => Block { x: x - 1, y },
            Direction::Right => Block { x: x + 1, y },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_any(&self, x: i32, y: i32) -> bool {
        self.body.iter().any(|b| b.x == x && b.y == y)
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut blocks = self
            .body
            .iter()
            .skip(1) // skip the head
            .peekable();
        loop {
            let (this_block, next) = (&blocks.next(), &blocks.peek());
            if this_block.is_none() || next.is_none() {
                return false;
            }
            if this_block.unwrap().x == x && this_block.unwrap().y == y {
                return true;
            }
        }
    }
}
