use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use crate::config::SnakeConfig;


pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    conf: SnakeConfig,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(conf: &SnakeConfig) -> Game {
        Game {
            snake: Snake::new(2, 2, conf.colors.snake),

            food_exists: true,
            food_x: 6,
            food_y: 4,

            conf: conf.clone(),

            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(self.conf.colors.food, self.food_x, self.food_y, con, g);
        }

        self.draw_borders(con, g);

        if self.game_over {
            draw_rectangle(self.conf.colors.game_over, 0, 0, self.conf.width, self.conf.height, con, g);
        }
    }

    fn draw_borders(&self, con: &Context, g: &mut G2d) {
        let (width, height) = (self.conf.width, self.conf.height);
        draw_rectangle(self.conf.colors.borders, 0, 0, width, 1, con, g);
        draw_rectangle(self.conf.colors.borders, 0, height - 1, width, 1, con, g);
        draw_rectangle(self.conf.colors.borders, 0, 0, 1, height, con, g);
        draw_rectangle(self.conf.colors.borders, width - 1, 0, 1, height, con, g);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > self.conf.restart_time {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.conf.moving_period {
            self.update_snake(None);
        }
    }
    fn handle_eating_food(&mut self) {
        if self.is_snake_eating() {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }
    fn is_snake_eating(&self) -> bool {
        let (head_x, head_y) = self.snake.head_position();
        self.food_exists && (head_x == self.food_x) && (head_y == self.food_y)
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // checking whether snake is about to hit border
        next_x > 0 && next_y > 0 && next_x < self.conf.width - 1 && next_y < self.conf.height - 1
    }

    fn add_food(&mut self) {
        let _new_pos = self.generate_new_food_position();
        while self.snake.overlap_tail(_new_pos.0, _new_pos.1) {
            let _new_pos = self.generate_new_food_position();
        }
        
        (self.food_x, self.food_y) = _new_pos;
        self.food_exists = true;
    }

    fn generate_new_food_position(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        let x = rng.gen_range(1..(self.conf.width - 1));
        let y = rng.gen_range(1..(self.conf.height - 1));
        (x, y)
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.handle_eating_food();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2, self.conf.colors.snake);

        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;

        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
