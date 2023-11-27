// extern crate piston_window;
// extern crate rand;
// extern crate 
mod config;
mod draw;
mod game;
mod snake;

use std::{fs, error};
use piston_window::*;


use draw::to_coord_u32;
use game::Game;
use config::SnakeConfig;

const CONFIG_PATH: &str = "resources/config.toml";

fn read_config_file(file_path: &str) -> Result<SnakeConfig, Box<dyn error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: SnakeConfig = toml::from_str(&contents)?;
    Ok(config)
}


fn main() {
    let conf = read_config_file(CONFIG_PATH).expect("could not load the config file.");
    // println!("{:?}", conf)
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(conf.width), to_coord_u32(conf.height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(&conf);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(conf.colors.background, g);
            game.draw(&c, g);
        });
        event.update(|arg| game.update(arg.dt));
    }
}
