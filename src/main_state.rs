use crate::{ball::*, controller::Controller, controller::ControllerInput, debug::DebugInfo, physics::*, racket::*, score::Score};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult, event, input::keyboard::KeyCode};
use std::collections::HashSet;

const MIDDLE_LINE_WIDTH: f32 = RACKET_WIDTH / 4.0;

pub struct MainState {
    player_left: Racket,
    player_right: Racket,
    ball: Ball,
    middle_line_mesh: Mesh,
    score: Score,
    debug: DebugInfo,
}

impl MainState {
    pub fn new(context: &mut Context, left_controller: Box<dyn Controller>, right_controller: Box<dyn Controller>) -> GameResult<Self> {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let (screen_width_center, screen_height_center) = (screen_width / 2.0, screen_height / 2.0);

        let middle_line_rectangle = Rect::new(
            context.gfx.drawable_size().0 / 2.0 - MIDDLE_LINE_WIDTH / 2.0,
            0.0,
            MIDDLE_LINE_WIDTH,
            context.gfx.drawable_size().1,
        );
        let middle_line_mesh = Mesh::new_rectangle(context, DrawMode::fill(), middle_line_rectangle, Color::from_rgb(127, 127, 127))?;

        let score = Score::new(context)?;

        Ok(MainState {
            player_left: Racket::new(RACKET_OFFSET, screen_height_center, context, left_controller)?,
            player_right: Racket::new(screen_width - RACKET_OFFSET, screen_height_center, context, right_controller)?,
            ball: Ball::new(screen_width_center, screen_height_center, context)?,
            middle_line_mesh,
            score,
            debug: DebugInfo::new(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let delta_time = context.time.delta().as_secs_f32();
        self.debug.update(context)?;

        self.debug.set_ball_info(context, self.ball.position, self.ball.velocity, self.ball.speed)?;

        if context.keyboard.is_key_just_pressed(KeyCode::F1) {
            self.debug.toggle();
        }

        // Move rackets (player 1: W/S, player 2: Up/Down)
        let mut pressed = HashSet::new();
        for k in context.keyboard.pressed_keys() {
            pressed.insert(*k);
        }

        let input_left = ControllerInput {
            ball_pos: self.ball.position,
            ball_vel: self.ball.velocity,
            racket_pos: self.player_left.pos_y,
            racket_x: self.player_left.pos_x,
            screen_height: context.gfx.drawable_size().1,
            pressed_keys: pressed.clone(),
        };

        let input_right = ControllerInput {
            ball_pos: self.ball.position,
            ball_vel: self.ball.velocity,
            racket_pos: self.player_right.pos_y,
            racket_x: self.player_right.pos_x,
            screen_height: context.gfx.drawable_size().1,
            pressed_keys: pressed,
        };

        self.player_left.update(&input_left, delta_time);
        self.player_right.update(&input_right, delta_time);

        bounce_borders(&mut self.ball, context.gfx.drawable_size().1);

        racket_collision(&mut self.ball, &self.player_left);
        racket_collision(&mut self.ball, &self.player_right);

        if let Some(scored) = check_score(&self.ball, context.gfx.drawable_size().0) {
            match scored {
                Player::Left => {
                    self.score.increment_p1(context)?;
                }
                Player::Right => {
                    self.score.increment_p2(context)?;
                }
            }
            self.ball.reset(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
        }

        self.ball.move_ball(delta_time);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(context, Color::BLACK);

        self.score.draw_on_canvas(&mut canvas);
        canvas.draw(&self.middle_line_mesh, DrawParam::default());
        self.player_left.draw_on_canvas(&mut canvas);
        self.player_right.draw_on_canvas(&mut canvas);
        self.ball.draw_on_canvas(&mut canvas);

        self.debug.draw(&mut canvas);

        canvas.finish(context)?;
        Ok(())
    }
}
