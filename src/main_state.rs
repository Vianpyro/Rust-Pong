use crate::ball::Ball;
use crate::racket::*;
use crate::score::Score;
use ggez::{GameResult, event, glam::Vec2, graphics};

const BALL_SPEED: f32 = 500.0;
const BALL_SIZE: f32 = 20.0;
const MIDDLE_LINE_WIDTH: f32 = RACKET_WIDTH / 4.0;

pub struct MainState {
    player_1: Racket,
    player_2: Racket,
    ball: Ball,
    middle_line_mesh: graphics::Mesh,
    score: Score,
}

impl MainState {
    pub fn new(context: &mut ggez::Context) -> GameResult<Self> {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let (screen_width_center, screen_height_center) = (screen_width / 2.0, screen_height / 2.0);

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

        let score = Score::new(context, score_position)?;

        Ok(MainState {
            player_1: Racket::new(RACKET_OFFSET, screen_height_center, context)?,
            player_2: Racket::new(screen_width - RACKET_OFFSET, screen_height_center, context)?,
            ball: Ball::new(screen_width_center, screen_height_center, context)?,
            middle_line_mesh,
            score,
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

        if self.ball.position.y - BALL_SIZE / 2.0 <= 0.0 && self.ball.velocity.y < 0.0
            || self.ball.position.y + BALL_SIZE / 2.0 >= context.gfx.drawable_size().1 && self.ball.velocity.y > 0.0
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Left racket collision
        if self.ball.position.x - BALL_SIZE / 2.0 <= self.player_1.pos_x + RACKET_WIDTH_HALF
            && self.ball.position.y >= self.player_1.pos_y - RACKET_HEIGHT_HALF
            && self.ball.position.y <= self.player_1.pos_y + RACKET_HEIGHT_HALF
            && self.ball.velocity.x < 0.0
        {
            self.ball.velocity.x = -self.ball.velocity.x;
            let offset = (self.ball.position.y - self.player_1.pos_y) / RACKET_HEIGHT_HALF;
            self.ball.velocity.y = BALL_SPEED * offset;

            self.ball.velocity = self.ball.velocity.normalize() * BALL_SPEED;
        }

        // Right racket collision
        if self.ball.position.x + BALL_SIZE / 2.0 >= self.player_2.pos_x - RACKET_WIDTH_HALF
            && self.ball.position.y >= self.player_2.pos_y - RACKET_HEIGHT_HALF
            && self.ball.position.y <= self.player_2.pos_y + RACKET_HEIGHT_HALF
            && self.ball.velocity.x > 0.0
        {
            self.ball.velocity.x = -self.ball.velocity.x;
            let offset = (self.ball.position.y - self.player_2.pos_y) / RACKET_HEIGHT_HALF;
            self.ball.velocity.y = BALL_SPEED * offset;

            self.ball.velocity = self.ball.velocity.normalize() * BALL_SPEED;
        }

        // Score update
        if self.ball.position.x < 0.0 {
            self.score.increment_p2();
            self.ball.reset(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
        }

        if self.ball.position.x > context.gfx.drawable_size().0 {
            self.score.increment_p1();
            self.ball.reset(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
        }

        self.ball.move_ball(delta_time);

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, graphics::Color::BLACK);

        self.score.draw_on_canvas(&mut canvas);
        canvas.draw(&self.middle_line_mesh, graphics::DrawParam::default());
        self.player_1.draw_on_canvas(&mut canvas);
        self.player_2.draw_on_canvas(&mut canvas);
        self.ball.draw_on_canvas(&mut canvas);

        canvas.finish(context)?;
        Ok(())
    }
}
