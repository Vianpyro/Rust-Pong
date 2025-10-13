use ggez::{Context, GameResult, glam::Vec2, graphics};
use graphics::{Canvas, Color, DrawParam, PxScale, Text};

pub struct DebugInfo {
    enabled: bool,
    fps_text: Text,
    position: Vec2,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            enabled: false,
            fps_text: Text::new(""),
            position: Vec2::new(10.0, 10.0),
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn update(&mut self, context: &Context) -> GameResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let fps = context.time.fps();
        let delta = context.time.delta().as_secs_f64() * 1000.0;

        self.fps_text = Text::new(format!("FPS: {:.0}\nFrame time: {:.2}ms", fps, delta));
        self.fps_text.set_scale(PxScale::from(20.0));

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if !self.enabled {
            return;
        }

        canvas.draw(&self.fps_text, DrawParam::default().dest(self.position).color(Color::from_rgb(0, 255, 0)));
    }
}
