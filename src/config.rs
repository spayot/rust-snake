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
