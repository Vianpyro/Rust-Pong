use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Text};
use ggez::{Context, GameResult, glam::Vec2};

pub struct DebugInfo {
    enabled: bool,
    fps_text: Text,
    position: Vec2,
    ball_position_text: Text,
    ball_velocity_text: Text,
    ball_speed_text: Text,
    ball_velocity_mesh: Option<Mesh>,
    ball_velocity_arrow_mesh: Option<Mesh>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            enabled: false,
            fps_text: Text::new(""),
            position: Vec2::new(10.0, 10.0),
            ball_position_text: Text::new(""),
            ball_velocity_text: Text::new(""),
            ball_speed_text: Text::new(""),
            ball_velocity_mesh: None,
            ball_velocity_arrow_mesh: None,
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

    pub fn set_ball_info(&mut self, context: &mut Context, position: Vec2, velocity: Vec2, speed: f32) -> GameResult<()> {
        self.ball_position_text = Text::new(format!("Ball position: {:.1}, {:.1}", position.x, position.y));
        self.ball_velocity_text = Text::new(format!("Ball velocity: {:.1}, {:.1}", velocity.x, velocity.y));
        self.ball_speed_text = Text::new(format!("Ball speed: {:.1}", speed));

        let scale = PxScale::from(18.0);
        self.ball_position_text.set_scale(scale);
        self.ball_velocity_text.set_scale(scale);
        self.ball_speed_text.set_scale(scale);

        let dir = if velocity.length() == 0.0 {
            Vec2::new(1.0, 0.0)
        } else {
            velocity.normalize()
        };
        let vel_length = 60.0;
        let end = position + dir * vel_length;
        self.ball_velocity_mesh = Some(Mesh::new_line(context, &[position, end], 3.0, Color::from_rgb(0, 255, 0))?);

        // Build an arrowhead (triangle) at the end of the velocity vector
        let arrow_tip = end + dir * 8.0;
        let arrow_back = end - dir * 6.0;
        let perp = Vec2::new(-dir.y, dir.x);
        let arrow_half_width = 6.0;
        let left = arrow_back + perp * arrow_half_width;
        let right = arrow_back - perp * arrow_half_width;
        let points = [arrow_tip, left, right];
        self.ball_velocity_arrow_mesh = Some(Mesh::new_polygon(context, DrawMode::fill(), &points, Color::from_rgb(0, 255, 0))?);

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if !self.enabled {
            return;
        }

        canvas.draw(&self.fps_text, DrawParam::default().dest(self.position).color(Color::from_rgb(0, 255, 0)));

        let line_height = 22.0;
        let pos = self.position;
        let pos_ball = Vec2::new(pos.x, pos.y + line_height * 2.0);
        let pos_vel = Vec2::new(pos.x, pos.y + line_height * 3.2);
        let pos_speed = Vec2::new(pos.x, pos.y + line_height * 4.4);

        canvas.draw(&self.ball_position_text, DrawParam::default().dest(pos_ball).color(Color::from_rgb(0, 255, 0)));
        canvas.draw(&self.ball_velocity_text, DrawParam::default().dest(pos_vel).color(Color::from_rgb(0, 255, 0)));
        canvas.draw(&self.ball_speed_text, DrawParam::default().dest(pos_speed).color(Color::from_rgb(0, 255, 0)));

        if let Some(mesh) = &self.ball_velocity_mesh {
            canvas.draw(mesh, DrawParam::default());
        }
        if let Some(mesh) = &self.ball_velocity_arrow_mesh {
            canvas.draw(mesh, DrawParam::default());
        }
    }
}
