use ggez::graphics::Canvas;
use ggez::{Context, GameResult};

use crate::debug::DebugInfo;

pub fn draw_hud(_context: &mut Context, canvas: &mut Canvas, debug: &DebugInfo) -> GameResult {
    // Delegate debug drawing to the DebugInfo helper
    debug.draw(canvas);
    Ok(())
}
