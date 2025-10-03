use ggez::{GameResult, glam::Vec2, graphics, input::keyboard::KeyCode};

const RACKET_SPEED: f32 = 650.0;

const RACKET_HEIGHT: f32 = 150.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT / 2.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH / 2.0;

pub struct Racket {
    pub pos_y: f32,
    pub pos_x: f32,
    racket_mesh: graphics::Mesh,
}

impl Racket {
    pub fn new(pos_x: f32, pos_y: f32, context: &mut ggez::Context) -> GameResult<Self> {
        let racket_rectangle = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), racket_rectangle, graphics::Color::WHITE)?;

        Ok(Self { pos_x, pos_y, racket_mesh })
    }

    pub fn move_racket(&mut self, up_key: KeyCode, down_key: KeyCode, context: &mut ggez::Context, delta_time: f32) {
        let racket_speed = RACKET_SPEED * delta_time;

        if context.keyboard.is_key_pressed(up_key) && self.pos_y - RACKET_HEIGHT_HALF > 0.0 {
            self.pos_y -= racket_speed;
        }

        if context.keyboard.is_key_pressed(down_key) && self.pos_y + RACKET_HEIGHT_HALF < context.gfx.drawable_size().1 {
            self.pos_y += racket_speed;
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        let draw_params = graphics::DrawParam::default().dest(Vec2::new(self.pos_x, self.pos_y));
        canvas.draw(&self.racket_mesh, draw_params);
    }
}
