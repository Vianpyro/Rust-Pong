mod main_state;

use crate::main_state::MainState;
use ggez::{GameResult, event};

const TITLE: &str = "Pong";

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new(TITLE, "Vianpyro");
    let (mut context, event_loop) = context_builder.build()?;
    context.gfx.set_window_title(TITLE);

    let state = MainState::new(&mut context);
    event::run(context, event_loop, state);
}
