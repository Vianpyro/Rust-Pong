use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::{Context, GameResult, glam::Vec2};

pub fn draw_pause_screen(context: &mut Context, canvas: &mut Canvas) -> GameResult {
    // Semi-transparent overlay
    let overlay_rect = Rect::new(0.0, 0.0, context.gfx.drawable_size().0, context.gfx.drawable_size().1);
    let overlay_mesh = Mesh::new_rectangle(context, DrawMode::fill(), overlay_rect, Color::from_rgba(0, 0, 0, 160))?;
    canvas.draw(&overlay_mesh, DrawParam::default());

    let (screen_width, screen_height) = context.gfx.drawable_size();

    // Paused title
    let mut title = Text::new("Paused");
    title.set_scale(screen_height / 10.0);
    let title_dimensions = title.measure(context)?;
    let title_position = Vec2::new((screen_width - title_dimensions.x) / 2.0, screen_height / 3.0);
    canvas.draw(&title, DrawParam::default().dest(title_position).color(Color::WHITE));

    // Hints
    let mut hint = Text::new("P: Resume   |   Esc: Menu");
    hint.set_scale(screen_height / 30.0);
    let hint_dimensions = hint.measure(context)?;
    let hint_position = Vec2::new((screen_width - hint_dimensions.x) / 2.0, screen_height * 0.65);
    canvas.draw(&hint, DrawParam::default().dest(hint_position).color(Color::from_rgb(200, 200, 200)));

    Ok(())
}
