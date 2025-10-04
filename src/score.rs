use ggez::{Context, GameResult, glam::Vec2, graphics};

pub struct Score {
    p1: u32,
    p2: u32,
    text: graphics::Text,
    pos: Vec2,
    scale: f32,
}

impl Score {
    pub fn new(context: &mut Context, pos: Vec2) -> GameResult<Self> {
        let p1 = 0;
        let p2 = 0;
        let scale = context.gfx.drawable_size().1 / 3.0;
        let mut text = graphics::Text::new(format!("{}   {}", p1, p2));
        text.set_scale(graphics::PxScale::from(scale));

        Ok(Self { p1, p2, text, pos, scale })
    }

    fn update_text(&mut self) {
        self.text = graphics::Text::new(format!("{}   {}", self.p1, self.p2));
        self.text.set_scale(graphics::PxScale::from(self.scale));
    }

    pub fn increment_p1(&mut self) {
        self.p1 = self.p1.saturating_add(1);
        self.update_text();
    }

    pub fn increment_p2(&mut self) {
        self.p2 = self.p2.saturating_add(1);
        self.update_text();
    }

    pub fn draw_on_canvas(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &self.text,
            graphics::DrawParam::default().dest(self.pos).color(graphics::Color::from_rgb(50, 50, 50)),
        );
    }
}
