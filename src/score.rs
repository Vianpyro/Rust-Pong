use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Text};
use ggez::{Context, GameResult, glam::Vec2};

pub struct Score {
    p1: u32,
    p2: u32,
    text: Text,
    position: Vec2,
    scale: f32,
}

impl Score {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let p1 = 0;
        let p2 = 0;
        let scale = context.gfx.drawable_size().1 / 3.0;
        let mut text = Text::new(format!("{}   {}", p1, p2));
        text.set_scale(PxScale::from(scale));
        let text_dimensions = text.measure(context)?;
        let position = Vec2::new(
            context.gfx.drawable_size().0 / 2.0 - text_dimensions.x / 2.0,
            context.gfx.drawable_size().1 / 2.0 - text_dimensions.y / 2.0,
        );

        Ok(Self { p1, p2, text, position, scale })
    }

    fn update_text(&mut self, context: &mut Context) -> GameResult<()> {
        self.text = Text::new(format!("{}   {}", self.p1, self.p2));
        self.text.set_scale(PxScale::from(self.scale));
        let text_dimensions = self.text.measure(context)?;
        self.position = Vec2::new(
            context.gfx.drawable_size().0 / 2.0 - text_dimensions.x / 2.0,
            context.gfx.drawable_size().1 / 2.0 - text_dimensions.y / 2.0,
        );
        Ok(())
    }

    pub fn increment_p1(&mut self, context: &mut Context) -> GameResult<()> {
        self.p1 = self.p1.saturating_add(1);
        self.update_text(context)
    }

    pub fn increment_p2(&mut self, context: &mut Context) -> GameResult<()> {
        self.p2 = self.p2.saturating_add(1);
        self.update_text(context)
    }

    pub fn draw_on_canvas(&self, canvas: &mut Canvas) {
        canvas.draw(&self.text, DrawParam::default().dest(self.position).color(Color::from_rgb(50, 50, 50)));
    }
}
