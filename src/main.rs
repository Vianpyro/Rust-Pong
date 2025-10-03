#![windows_subsystem = "windows"]

mod main_state;

use crate::main_state::MainState;
use ggez::{
    GameResult,
    conf::{FullscreenType, WindowMode},
    event,
};

const TITLE: &str = "Pong";

fn main() -> GameResult {
    let window_mode = WindowMode::default().fullscreen_type(FullscreenType::Desktop);
    let context_builder = ggez::ContextBuilder::new(TITLE, "Vianpyro").window_mode(window_mode);

    let (mut context, event_loop) = context_builder.build()?;
    context.gfx.set_window_title(TITLE);

    let state = MainState::new(&mut context)?;
    event::run(context, event_loop, state);
}
