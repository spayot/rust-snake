use piston_window::types::Color;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct SnakeConfig {
    pub colors: ColorConfig,
    pub width: i32,
    pub height: i32,
    pub moving_period: f64,
    pub restart_time: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ColorConfig {
    pub snake: Color,
    pub background: Color,
    pub food: Color,
    pub borders: Color,
    pub game_over: Color,
}

// pub const SNAKE_COLOR: Color = [0.8, 0.0, 0.0, 1.0];

// pub const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
// pub const WIDTH: i32 = 20;
// pub const HEIGHT: i32 = 20;

// pub const FOOD_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
// pub const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
// pub const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

// pub const MOVING_PERIOD: f64 = 0.1;
// pub const RESTART_TIME: f64 = 1.0;

