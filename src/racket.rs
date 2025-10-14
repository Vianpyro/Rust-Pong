use crate::controller::{Controller, RacketAction::*};
use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

const RACKET_SPEED: f32 = 650.0;
pub const RACKET_HEIGHT: f32 = 150.0;
pub const RACKET_WIDTH: f32 = 20.0;
pub const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT / 2.0;
pub const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH / 2.0;
pub const RACKET_OFFSET: f32 = RACKET_WIDTH * 2.0;

pub struct Racket {
    pub pos_y: f32,
    pub pos_x: f32,
    racket_mesh: Mesh,
    pub controller: Box<dyn Controller>,
}

impl Racket {
    pub fn new(x: f32, y: f32, context: &mut Context, controller: Box<dyn Controller>) -> GameResult<Self> {
        let rect = Rect::new(-RACKET_WIDTH / 2.0, -RACKET_HEIGHT / 2.0, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = Mesh::new_rectangle(context, ggez::graphics::DrawMode::fill(), rect, Color::WHITE)?;

        Ok(Self {
            pos_x: x,
            pos_y: y,
            racket_mesh,
            controller,
        })
    }

    pub fn draw_on_canvas(&self, canvas: &mut Canvas) {
        canvas.draw(&self.racket_mesh, DrawParam::default().dest([self.pos_x, self.pos_y]));
    }

    pub fn update(&mut self, input: &crate::controller::ControllerInput, delta_time: f32) {
        match self.controller.get_action(input) {
            MoveUp => {
                self.pos_y -= RACKET_SPEED * delta_time;
            }
            MoveDown => {
                self.pos_y += RACKET_SPEED * delta_time;
            }
            Stay => {}
        }

        // Keep the racket inside the screen bounds
        let half_height = RACKET_HEIGHT / 2.0;
        if self.pos_y < half_height {
            self.pos_y = half_height;
        }
        let lower_limit = input.screen_height - half_height;
        if self.pos_y > lower_limit {
            self.pos_y = lower_limit;
        }
    }
}
