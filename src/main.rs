#![windows_subsystem = "windows"]

mod audio;
mod ball;
mod controller;
mod debug;
mod main_state;
mod physics;
mod player_type;
mod racket;
mod score;
mod ui;

use crate::main_state::MainState;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::{ContextBuilder, GameResult, event};

const TITLE: &str = "Pong";

fn main() -> GameResult {
    let window_mode = WindowMode::default().fullscreen_type(FullscreenType::Desktop);
    let context_builder = ContextBuilder::new(TITLE, "Vianpyro").window_mode(window_mode).add_resource_path("./assets");

    let (mut context, event_loop) = context_builder.build()?;
    context.gfx.set_window_title(TITLE);

    let state = MainState::new(&mut context)?;
    event::run(context, event_loop, state);
}
