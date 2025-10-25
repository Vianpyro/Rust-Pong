use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::{Context, GameResult, glam::Vec2};

use crate::game::physics::Player;

pub fn draw_game_over(context: &mut Context, canvas: &mut Canvas, winner: Player) -> GameResult {
    // Semi-transparent overlay
    let overlay_rect = Rect::new(0.0, 0.0, context.gfx.drawable_size().0, context.gfx.drawable_size().1);
    let overlay_mesh = Mesh::new_rectangle(context, DrawMode::fill(), overlay_rect, Color::from_rgba(0, 0, 0, 180))?;
    canvas.draw(&overlay_mesh, DrawParam::default());

    let (screen_width, screen_height) = context.gfx.drawable_size();

    // Winner text
    let winner_text = match winner {
        Player::Left => "Player 1 Wins!",
        Player::Right => "Player 2 Wins!",
    };

    super::common::draw_centered_title(context, canvas, winner_text, Color::WHITE)?;

    // Press to continue
    let mut continue_text = Text::new("R: Restart  |   Esc: Menu");
    continue_text.set_scale(screen_height / 30.0);
    let continue_dimensions = continue_text.measure(context)?;
    let continue_position = Vec2::new((screen_width - continue_dimensions.x) / 2.0, screen_height * 0.65);
    canvas.draw(
        &continue_text,
        DrawParam::default().dest(continue_position).color(Color::from_rgb(200, 200, 200)),
    );

    Ok(())
}
