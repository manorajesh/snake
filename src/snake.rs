use std::{hash::Hash, collections::HashSet};

use rand::{self, Rng};

use crate::{WIDTH, HEIGHT};

pub struct Game {
    snake: Snake,
    food: Food,
    bounds: (u32, u32),
}

#[derive(Clone, PartialEq, Eq)]
struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

#[derive(PartialEq)]
struct Food {
    position: Position,
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake {
                body: vec![Position { x: 0, y: 0 }],
                direction: Direction::Right,
            },
            food: Food { position: Position { x: 5, y: 5 } },

            bounds: (WIDTH/10, HEIGHT/10),
        }
    }

    // return true if game is over
    pub fn update(&mut self) -> bool {
        // Update snake position
        self.snake.move_body();

        // Check for collision
        if self.snake.body[0].x >= self.bounds.0 || self.snake.body[0].y >= self.bounds.1 {
            return true;
        }

        if !has_unique_elements(&self.snake.body) {
            return true;
        }

        // Check for food
        if self.snake.body[0] == self.food.position {
            self.snake.body.push(self.snake.body[self.snake.body.len() - 1].clone());
            self.food.position = Position {
                x: rand::thread_rng().gen_range(0..self.bounds.0),
                y: rand::thread_rng().gen_range(0..self.bounds.1),
            };
        }
        return false;
    }

    pub fn draw(&self, frame: &mut [u8], width: u32) {
        let scale: usize = width as usize / self.bounds.0 as usize;
    
        // clear the frame
        frame.fill(0);
    
        for y in 0..self.bounds.1 {
            for x in 0..self.bounds.0 {
                let rgba = if self.snake.body.contains(&Position { x, y }) {
                    [0x00, 0xff, 0x00, 0xff]
                } else if self.food.position ==( Position { x, y }) {
                    [0xff, 0x00, 0x00, 0xff]
                } else {
                    [0x00, 0x00, 0x00, 0xff]
                };
    
                for i in 0..scale {
                    for j in 0..scale {
                        let idx = ((y as usize * scale + i) * width as usize + x as usize * scale + j) * 4;
                        frame[idx..idx + 4].copy_from_slice(&rgba);
                    }
                }
            }
        }
    }
    

    pub fn change_direction(&mut self, dir: Direction) {
        match (&self.snake.direction, &dir) {
            (Direction::Up, Direction::Down) | 
            (Direction::Down, Direction::Up) | 
            (Direction::Left, Direction::Right) | 
            (Direction::Right, Direction::Left) => {
                // If the new direction is the reverse of the current direction,
                // then ignore this direction change by exiting the function early.
                return;
            }
            _ => {
                // Otherwise, change the direction as usual.
                self.snake.direction = dir;
            }
        }
    }
    

    pub fn resize(&mut self, width: u32, height: u32) {
        self.bounds = (width / 10, height / 10);
    }
}

impl Snake {
    fn move_body(&mut self) {
        let mut new_head = self.body[0].clone();
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.insert(0, new_head);

        // We don't want to pop the last element if the snake has just eaten
        if self.body.len() > 2 {
            self.body.pop();
        }
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}