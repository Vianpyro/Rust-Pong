use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::{Context, GameResult, glam::Vec2};

pub fn draw_centered_title(context: &mut Context, canvas: &mut Canvas, text: &str, color: Color) -> GameResult {
    let (screen_width, screen_height) = context.gfx.drawable_size();
    let mut title = Text::new(text);
    title.set_scale(screen_height / 10.0);
    let title_dimensions = title.measure(context)?;
    let title_position = Vec2::new((screen_width - title_dimensions.x) / 2.0, screen_height / 3.0);
    canvas.draw(&title, DrawParam::default().dest(title_position).color(color));
    Ok(())
}
