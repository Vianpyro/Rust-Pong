use ggez::{GameResult, event, glam::Vec2, graphics, input::keyboard::KeyCode};
use rand::Rng;

const RACKET_SPEED: f32 = 1.0;
const BALL_SPEED: f32 = 100.0;
const RACKET_HEIGHT: f32 = 150.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT / 2.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH / 2.0;
const RACKET_OFFSET: f32 = RACKET_WIDTH;
const BALL_SIZE: f32 = 20.0;
const MIDDLE_LINE_WIDTH: f32 = RACKET_WIDTH / 4.0;

fn move_racket(position: &mut Vec2, up_key: KeyCode, down_key: KeyCode, context: &mut ggez::Context, delta_time: f32) {
    let racket_speed = RACKET_SPEED * delta_time * 1000.0;

    if context.keyboard.is_key_pressed(up_key) && position.y - RACKET_HEIGHT_HALF > 0.0 {
        position.y -= racket_speed;
    }

    if context.keyboard.is_key_pressed(down_key) && position.y + RACKET_HEIGHT_HALF < context.gfx.drawable_size().1 {
        position.y += racket_speed;
    }
}

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
    player_1_position: Vec2,
    player_2_position: Vec2,
    player_1_score: u32,
    player_2_score: u32,
    ball_position: Vec2,
    ball_velocity: Vec2,
}

impl MainState {
    pub fn new(context: &mut ggez::Context) -> Self {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let (screen_width_center, screen_height_center) = (screen_width / 2.0, screen_height / 2.0);

        let mut ball_velocity = Vec2::new(0.0, 0.0);
        randomize_velocity(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_position: Vec2::new(RACKET_OFFSET, screen_height_center),
            player_2_position: Vec2::new(screen_width - RACKET_OFFSET, screen_height_center),
            player_1_score: 0,
            player_2_score: 0,
            ball_position: Vec2::new(screen_width_center, screen_height_center),
            ball_velocity,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        let delta_time = context.time.delta().as_secs_f32();

        move_racket(&mut self.player_1_position, KeyCode::W, KeyCode::S, context, delta_time);
        move_racket(&mut self.player_2_position, KeyCode::Up, KeyCode::Down, context, delta_time);

        if self.ball_position.y - BALL_SIZE / 2.0 <= 0.0 || self.ball_position.y + BALL_SIZE / 2.0 >= context.gfx.drawable_size().1 {
            self.ball_velocity.y = -self.ball_velocity.y;
        }

        if self.ball_position.x - BALL_SIZE / 2.0 <= self.player_1_position.x + RACKET_WIDTH_HALF
            && self.ball_position.y >= self.player_1_position.y - RACKET_HEIGHT_HALF
            && self.ball_position.y <= self.player_1_position.y + RACKET_HEIGHT_HALF
        {
            self.ball_velocity.x = -self.ball_velocity.x;
            let offset = (self.ball_position.y - self.player_1_position.y) / RACKET_HEIGHT_HALF;
            self.ball_velocity.y = BALL_SPEED * offset;
        }

        if self.ball_position.x + BALL_SIZE / 2.0 >= self.player_2_position.x - RACKET_WIDTH_HALF
            && self.ball_position.y >= self.player_2_position.y - RACKET_HEIGHT_HALF
            && self.ball_position.y <= self.player_2_position.y + RACKET_HEIGHT_HALF
        {
            self.ball_velocity.x = -self.ball_velocity.x;
            let offset = (self.ball_position.y - self.player_2_position.y) / RACKET_HEIGHT_HALF;
            self.ball_velocity.y = BALL_SPEED * offset;
        }

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

        let racket_rectangle = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), racket_rectangle, graphics::Color::WHITE)?;

        let ball_rectangle = graphics::Rect::new(-BALL_SIZE / 2.0, -BALL_SIZE / 2.0, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), ball_rectangle, graphics::Color::WHITE)?;

        // Draw scores
        let mut score_text = graphics::Text::new(format!("{}   {}", self.player_1_score, self.player_2_score));
        score_text.set_scale(graphics::PxScale::from(context.gfx.drawable_size().1 / 3.0));
        let text_dimensions = score_text.measure(context)?;
        let score_position = Vec2::new(
            context.gfx.drawable_size().0 / 2.0 - text_dimensions.x / 2.0,
            context.gfx.drawable_size().1 / 2.0 - text_dimensions.y / 2.0,
        );
        canvas.draw(
            &score_text,
            graphics::DrawParam::default().dest(score_position).color(graphics::Color::from_rgb(50, 50, 50)),
        );

        // Draw middle line
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
        canvas.draw(&middle_line_mesh, graphics::DrawParam::default());

        // Draw rackets and ball
        canvas.draw(&racket_mesh, graphics::DrawParam::default().dest(self.player_1_position));
        canvas.draw(&racket_mesh, graphics::DrawParam::default().dest(self.player_2_position));
        canvas.draw(&ball_mesh, graphics::DrawParam::default().dest(self.ball_position));

        canvas.finish(context)?;
        Ok(())
    }
}
