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
    pub opponent_pos: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub pressed_keys: HashSet<KeyCode>,
}

impl ControllerInput {
    // Convert the input to a simple float feature vector suitable for feeding into a neural network or ML model.
    pub fn to_feature_vec(&self) -> Vec<f32> {
        vec![
            self.ball_pos.x,
            self.ball_pos.y,
            self.ball_vel.x,
            self.ball_vel.y,
            self.racket_pos,
            self.racket_x,
            self.opponent_pos,
            self.screen_height,
        ]
    }
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

pub struct AIController {}

impl AIController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for AIController {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction {
        let top = input.racket_pos - RACKET_HEIGHT_HALF;
        let bottom = input.racket_pos + RACKET_HEIGHT_HALF;
        let approaching = (input.ball_vel.x > 0.0 && input.racket_x > input.ball_pos.x && input.screen_width / 2.0 < input.ball_pos.x)
            || (input.ball_vel.x < 0.0 && input.racket_x < input.ball_pos.x && input.screen_width / 2.0 > input.ball_pos.x);

        if approaching {
            if input.ball_pos.y < top {
                RacketAction::MoveUp
            } else if input.ball_pos.y > bottom {
                RacketAction::MoveDown
            } else {
                RacketAction::Stay
            }
        } else {
            let center = input.screen_height / 2.0;
            let deadzone = RACKET_HEIGHT_HALF;
            if input.racket_pos < center - deadzone {
                RacketAction::MoveDown
            } else if input.racket_pos > center + deadzone {
                RacketAction::MoveUp
            } else {
                RacketAction::Stay
            }
        }
    }
}
