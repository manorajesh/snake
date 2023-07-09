struct Game {
    snake: Snake,
    food: Food,
    bounds: (i32, i32),
}

#[derive(Clone)]
struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

struct Food {
    position: Position,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum Direction {
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

            bounds: (20, 20),
        }
    }

    pub fn update(&mut self, input: Option<Direction>) -> bool {
        // Update snake position
        if let Some(dir) = input {
            self.snake.direction = dir;
        }
        self.snake.move_body();

        // Check for collision
        if self.snake.body[0].x < 0 || self.snake.body[0].x >= self.bounds.0 ||
            self.snake.body[0].y < 0 || self.snake.body[0].y >= self.bounds.1 {
            return false;
        }

        // Check for food
        return true;
    }
}

impl Snake {
    fn move_body(&mut self) {
        let mut head = &mut self.body[0];
        match self.direction {
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        }

        drop(head);

        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1].clone();
        }
    }
}