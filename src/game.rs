use crate::draw::{draw_block, draw_rectangle};
use crate::snakes::Snake;
use piston_window::keyboard::Key;
use piston_window::types::Color;
use piston_window::{color, Context, G2d};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

enum GameState {
    Running,
    PressKeyToStart,
}

pub struct Game {
    snake: Snake,
    border_color: Color,
    state: GameState,

    time_until_next_move: f64,

    width: i32,
    height: i32,

    foods: Vec<Food>,
    foods_eaten: i32,

    rng: ThreadRng,
}

#[derive(Copy, Clone)]
struct Food {
    x: i32,
    y: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::new(3, 3);
        let time_until_next_move = 1. / &snake.speed;
        Game {
            snake,
            border_color: color::WHITE,
            state: GameState::Running,
            time_until_next_move,
            width,
            height,
            foods: Vec::new(),
            foods_eaten: 0,
            rng: thread_rng(),
        }
    }

    pub fn key_down(&mut self, key: Key) {
        match self.state {
            GameState::PressKeyToStart => {
                self.reset_game();
            }

            GameState::Running => {
                match key {
                    Key::Left => self.snake.turn_left(),
                    Key::Right => self.snake.turn_right(),
                    Key::Up => self.snake.turn_up(),
                    Key::Down => self.snake.turn_down(),
                    _ => (),
                };
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            GameState::PressKeyToStart => {}

            GameState::Running => {
                self.time_until_next_move -= dt;
                if self.time_until_next_move <= 0. {
                    self.snake.move_forward();
                    self.time_until_next_move += 1. / self.snake.speed;
                }

                if self.foods.is_empty() {
                    self.spawn_food();
                    if self.foods_eaten > 3 {
                        self.spawn_food();
                    }
                } else {
                    self.check_for_eaten_food();
                }

                let (sx, sy) = self.snake.head_position();
                if sx <= 1 || sx >= self.width - 1 || sy <= 1 || sy >= self.height - 1 {
                    println!("out of bounds");
                    self.game_over();
                }
                if self.snake.overlap_tail(sx, sy) {
                    println!("overlap tail");
                    self.game_over();
                }
            }
        }
    }

    fn game_over(&mut self) {
        self.border_color = color::RED;
        self.state = GameState::PressKeyToStart;
    }

    fn reset_game(&mut self) {
        self.snake = Snake::new(3, 3);
        self.time_until_next_move = 1. / self.snake.speed;
        self.foods_eaten = 0;
        self.foods = Vec::new();
        self.border_color = color::WHITE;
        self.state = GameState::Running;
    }

    fn check_for_eaten_food(&mut self) {
        let mut eaten_food_index: Option<usize> = None;
        for (index, food) in self.foods.iter().enumerate() {
            if self.snake.head_position() == (food.x, food.y) {
                // eat food
                eaten_food_index = Some(index);
                self.foods_eaten += 1;
                self.snake.restore_tail();
                self.snake.speed += 1.;
            }
        }
        if let Some(index) = eaten_food_index {
            self.foods.remove(index);
        }
    }

    pub fn render(&self, context: &Context, graphics: &mut G2d) {
        self.draw_border(self.border_color, &context, graphics);
        self.snake.draw(&context, graphics);

        for food in &self.foods {
            draw_block(color::RED, food.x, food.y, &context, graphics);
        }
    }

    fn draw_border(&self, color: Color, context: &Context, graphics: &mut G2d) {
        // top side
        draw_rectangle(color, 0, 0, self.width, 1, &context, graphics);

        // right side
        draw_rectangle(color, self.width - 1, 0, 1, self.height, &context, graphics);

        // bottom side
        draw_rectangle(color, 0, self.height - 1, self.width, 1, &context, graphics);

        // left side
        draw_rectangle(color, 0, 0, 1, self.height, &context, graphics);
    }

    fn spawn_food(&mut self) {
        loop {
            let x: i32 = self.rng.gen_range(2..self.width - 2);
            let y: i32 = self.rng.gen_range(2..self.height - 2);

            if self.snake.overlap_any(x, y) || self.foods.iter().any(|f| (f.x, f.y) == (x, y)) {
                continue;
            }

            self.foods.push(Food { x, y });
            return;
        }
    }
}
