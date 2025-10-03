use crate::racket::*;
use ggez::{GameResult, event, glam::Vec2, graphics};
use rand::Rng;

const BALL_SPEED: f32 = 500.0;
const BALL_SIZE: f32 = 20.0;
const MIDDLE_LINE_WIDTH: f32 = RACKET_WIDTH / 4.0;

fn randomize_velocity(vector: &mut Vec2, x: f32, y: f32) {
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

pub struct MainState {
    player_1: Racket,
    player_2: Racket,
    player_1_score: u32,
    player_2_score: u32,
    ball_position: Vec2,
    ball_velocity: Vec2,

    // Meshes
    ball_mesh: graphics::Mesh,
    middle_line_mesh: graphics::Mesh,
    score_text: graphics::Text,
    score_position: Vec2,
}

impl MainState {
    pub fn new(context: &mut ggez::Context) -> GameResult<Self> {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let (screen_width_center, screen_height_center) = (screen_width / 2.0, screen_height / 2.0);

        let mut ball_velocity = Vec2::new(0.0, 0.0);
        randomize_velocity(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        let player_1_score = 0;
        let player_2_score = 0;

        // Meshes
        let ball_rectangle = graphics::Rect::new(-BALL_SIZE / 2.0, -BALL_SIZE / 2.0, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), ball_rectangle, graphics::Color::WHITE)?;

        let middle_line_rectangle = graphics::Rect::new(
            context.gfx.drawable_size().0 / 2.0 - MIDDLE_LINE_WIDTH / 2.0,
            0.0,
            MIDDLE_LINE_WIDTH,
            context.gfx.drawable_size().1,
        );
        let middle_line_mesh = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            middle_line_rectangle,
            graphics::Color::from_rgb(127, 127, 127),
        )?;

        let mut score_text = graphics::Text::new(format!("{}   {}", player_1_score, player_2_score));
        score_text.set_scale(graphics::PxScale::from(context.gfx.drawable_size().1 / 3.0));
        let text_dimensions = score_text.measure(context)?;
        let score_position = Vec2::new(
            context.gfx.drawable_size().0 / 2.0 - text_dimensions.x / 2.0,
            context.gfx.drawable_size().1 / 2.0 - text_dimensions.y / 2.0,
        );
        let player_1 = Racket::new(RACKET_OFFSET, screen_height_center, context)?;
        let player_2 = Racket::new(screen_width - RACKET_OFFSET, screen_height_center, context)?;

        Ok(MainState {
            player_1,
            player_2,
            player_1_score,
            player_2_score,
            ball_position: Vec2::new(screen_width_center, screen_height_center),
            ball_velocity: ball_velocity.normalize() * BALL_SPEED,
            ball_mesh,
            middle_line_mesh,
            score_text,
            score_position,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        let delta_time = context.time.delta().as_secs_f32();

        // Move rackets (player 1: W/S, player 2: Up/Down)
        self.player_1
            .move_racket(ggez::input::keyboard::KeyCode::W, ggez::input::keyboard::KeyCode::S, context, delta_time);
        self.player_2
            .move_racket(ggez::input::keyboard::KeyCode::Up, ggez::input::keyboard::KeyCode::Down, context, delta_time);

        if self.ball_position.y - BALL_SIZE / 2.0 <= 0.0 && self.ball_velocity.y < 0.0
            || self.ball_position.y + BALL_SIZE / 2.0 >= context.gfx.drawable_size().1 && self.ball_velocity.y > 0.0
        {
            self.ball_velocity.y = -self.ball_velocity.y;
        }

        // Left racket collision
        if self.ball_position.x - BALL_SIZE / 2.0 <= self.player_1.pos_x + RACKET_WIDTH_HALF
            && self.ball_position.y >= self.player_1.pos_y - RACKET_HEIGHT_HALF
            && self.ball_position.y <= self.player_1.pos_y + RACKET_HEIGHT_HALF
            && self.ball_velocity.x < 0.0
        {
            self.ball_velocity.x = -self.ball_velocity.x;
            let offset = (self.ball_position.y - self.player_1.pos_y) / RACKET_HEIGHT_HALF;
            self.ball_velocity.y = BALL_SPEED * offset;

            self.ball_velocity = self.ball_velocity.normalize() * BALL_SPEED;
        }

        // Right racket collision
        if self.ball_position.x + BALL_SIZE / 2.0 >= self.player_2.pos_x - RACKET_WIDTH_HALF
            && self.ball_position.y >= self.player_2.pos_y - RACKET_HEIGHT_HALF
            && self.ball_position.y <= self.player_2.pos_y + RACKET_HEIGHT_HALF
            && self.ball_velocity.x > 0.0
        {
            self.ball_velocity.x = -self.ball_velocity.x;
            let offset = (self.ball_position.y - self.player_2.pos_y) / RACKET_HEIGHT_HALF;
            self.ball_velocity.y = BALL_SPEED * offset;

            self.ball_velocity = self.ball_velocity.normalize() * BALL_SPEED;
        }

        // Score update
        if self.ball_position.x < 0.0 {
            self.player_2_score += 1;
            self.ball_position = Vec2::new(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
            randomize_velocity(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
        }

        if self.ball_position.x > context.gfx.drawable_size().0 {
            self.player_1_score += 1;
            self.ball_position = Vec2::new(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
            randomize_velocity(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
        }

        self.ball_position += self.ball_velocity * delta_time;

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, graphics::Color::BLACK);

        canvas.draw(
            &self.score_text,
            graphics::DrawParam::default()
                .dest(self.score_position)
                .color(graphics::Color::from_rgb(50, 50, 50)),
        );
        canvas.draw(&self.middle_line_mesh, graphics::DrawParam::default());
        self.player_1.draw_on_canvas(&mut canvas);
        self.player_2.draw_on_canvas(&mut canvas);
        canvas.draw(&self.ball_mesh, graphics::DrawParam::default().dest(self.ball_position));

        canvas.finish(context)?;
        Ok(())
    }
}
