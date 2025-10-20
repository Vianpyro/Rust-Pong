use ggez::{Context, GameResult, glam::Vec2, graphics};
use rand::Rng;

pub const BALL_SPEED: f32 = 750.0;
pub const BALL_SIZE: f32 = 20.0;
pub const BALL_SPEED_INCREMENT: f32 = 1.05;
pub const BALL_SPEED_MAX: f32 = 2500.0;

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    ball_mesh: graphics::Mesh,
}

pub fn randomize_velocity(vector: &mut Vec2, x: f32, y: f32) {
    let mut random_thread = rand::rng();
    vector.x = match random_thread.random_bool(0.5) {
        true => x,
        false => -x,
    };
    vector.y = match random_thread.random_bool(0.5) {
        true => y,
        false => -y,
    };
}

impl Ball {
    /// Draw the ball on the provided canvas.
    pub fn draw_on_canvas(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(&self.ball_mesh, graphics::DrawParam::default().dest(self.position));
    }

    /// Reset ball position and speed, and randomize its direction.
    pub fn reset(&mut self, position_x: f32, position_y: f32) {
        self.position = Vec2::new(position_x, position_y);
        self.speed = BALL_SPEED;
        randomize_velocity(&mut self.velocity, self.speed, self.speed);
        self.velocity = self.velocity.normalize() * self.speed;
    }

    pub fn new(position_x: f32, position_y: f32, context: &mut Context) -> GameResult<Self> {
        let mut ball_velocity = Vec2::new(0.0, 0.0);
        randomize_velocity(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        let ball_rectangle = graphics::Rect::new(-BALL_SIZE / 2.0, -BALL_SIZE / 2.0, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), ball_rectangle, graphics::Color::WHITE)?;
        Ok(Ball {
            position: Vec2::new(position_x, position_y),
            velocity: ball_velocity.normalize() * BALL_SPEED,
            speed: BALL_SPEED,
            ball_mesh,
        })
    }

    pub fn move_ball(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}
