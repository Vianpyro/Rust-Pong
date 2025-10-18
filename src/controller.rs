use crate::racket::RACKET_HEIGHT_HALF;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use std::collections::HashSet;

pub trait Controller {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction;
}

pub enum RacketAction {
    MoveUp,
    MoveDown,
    Stay,
}

pub struct ControllerInput {
    pub ball_pos: Vec2,
    pub ball_vel: Vec2,
    pub racket_pos: f32,
    pub racket_x: f32,
    pub screen_height: f32,
    pub pressed_keys: HashSet<KeyCode>,
}

pub struct HumanController {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

impl HumanController {
    pub fn new(up_key: KeyCode, down_key: KeyCode) -> Self {
        Self { up_key, down_key }
    }
}

impl Controller for HumanController {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction {
        if input.pressed_keys.contains(&self.up_key) {
            RacketAction::MoveUp
        } else if input.pressed_keys.contains(&self.down_key) {
            RacketAction::MoveDown
        } else {
            RacketAction::Stay
        }
    }
}

trait AiBehavior {
    /// Choose a vertical target (y) for the racket based on the controller input.
    fn choose_target(&mut self, input: &ControllerInput) -> f32;
}

struct ReactiveBehavior {}

impl ReactiveBehavior {
    fn new() -> Self {
        Self {}
    }
}

impl AiBehavior for ReactiveBehavior {
    fn choose_target(&mut self, input: &ControllerInput) -> f32 {
        input.ball_pos.y
    }
}

struct PredictiveBehavior {}

impl PredictiveBehavior {
    fn new() -> Self {
        Self {}
    }

    /// Predict where the ball will be vertically when it reaches racket_x.
    fn predict_ball_y(&self, input: &ControllerInput) -> f32 {
        // time until ball reaches racket x
        let delta_x = input.racket_x - input.ball_pos.x;
        if input.ball_vel.x == 0.0 {
            return input.ball_pos.y;
        }
        let time_to_reach = delta_x / input.ball_vel.x;

        // projected vertical position at that time (may be outside bounds)
        let projected_y = input.ball_pos.y + input.ball_vel.y * time_to_reach;

        // reflect across top/bottom using mirror modulus to account for bounces
        let screen_height = input.screen_height;
        if screen_height <= 0.0 {
            return projected_y;
        }
        let wrap_period = 2.0 * screen_height;
        let mut modded = projected_y % wrap_period;
        if modded < 0.0 {
            modded += wrap_period;
        }
        if modded <= screen_height { modded } else { 2.0 * screen_height - modded }
    }
}

impl AiBehavior for PredictiveBehavior {
    fn choose_target(&mut self, input: &ControllerInput) -> f32 {
        self.predict_ball_y(input)
    }
}

pub struct AIController {
    strategy: Box<dyn AiBehavior + Send>,
}

impl AIController {
    pub fn easy() -> Self {
        Self {
            strategy: Box::new(ReactiveBehavior::new()),
        }
    }

    pub fn expert() -> Self {
        Self {
            strategy: Box::new(PredictiveBehavior::new()),
        }
    }
}

impl Controller for AIController {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction {
        let racket_top = input.racket_pos - RACKET_HEIGHT_HALF;
        let racket_bottom = input.racket_pos + RACKET_HEIGHT_HALF;

        // Decide whether the ball is approaching this racket (works for either side)
        let ball_approaching = (input.ball_vel.x > 0.0 && input.racket_x > input.ball_pos.x) || (input.ball_vel.x < 0.0 && input.racket_x < input.ball_pos.x);

        if ball_approaching {
            let target_y = self.strategy.choose_target(input);
            if target_y < racket_top {
                RacketAction::MoveUp
            } else if target_y > racket_bottom {
                RacketAction::MoveDown
            } else {
                RacketAction::Stay
            }
        } else {
            let center_y = input.screen_height / 2.0;
            let deadzone = RACKET_HEIGHT_HALF;
            if input.racket_pos < center_y - deadzone {
                RacketAction::MoveDown
            } else if input.racket_pos > center_y + deadzone {
                RacketAction::MoveUp
            } else {
                RacketAction::Stay
            }
        }
    }
}
