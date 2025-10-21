use crate::racket::RACKET_HEIGHT_HALF;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use std::collections::HashSet;

const AI_RACKET_PERCEPTION: f32 = 0.75;

pub trait Controller {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction;
}

pub enum RacketAction {
    MoveUp,
    MoveDown,
    Stay,
}

pub struct ControllerInput {
    pub ball_position: Vec2,
    pub ball_velocity: Vec2,
    pub racket_position: f32,
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
        input.ball_position.y
    }
}

pub struct PredictiveBehavior {}

impl PredictiveBehavior {
    fn new() -> Self {
        Self {}
    }

    /// Predict where the ball will be vertically when it reaches racket_x.
    pub fn predict_ball_y(&self, input: &ControllerInput) -> f32 {
        // time until ball reaches racket x
        let delta_x = input.racket_x - input.ball_position.x;
        if input.ball_velocity.x == 0.0 {
            return input.ball_position.y;
        }
        let time_to_reach = delta_x / input.ball_velocity.x;

        // projected vertical position at that time (may be outside bounds)
        let projected_y = input.ball_position.y + input.ball_velocity.y * time_to_reach;

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

pub struct BalancedBehavior {
    // Fields to store necessary state
}

impl BalancedBehavior {
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }
}

impl AiBehavior for BalancedBehavior {
    fn choose_target(&mut self, input: &ControllerInput) -> f32 {
        // Logic to average between Reactive and Predictive behavior
        let reactive_target = ReactiveBehavior::new().choose_target(input);
        let predictive_target = PredictiveBehavior::new().choose_target(input);
        (reactive_target + predictive_target) / 2.0
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

    pub fn medium() -> Self {
        Self {
            strategy: Box::new(BalancedBehavior::new()),
        }
    }

    pub fn hard() -> Self {
        Self {
            strategy: Box::new(PredictiveBehavior::new()),
        }
    }
}

impl Controller for AIController {
    fn get_action(&mut self, input: &ControllerInput) -> RacketAction {
        let perceived_half_height = RACKET_HEIGHT_HALF * AI_RACKET_PERCEPTION;

        let racket_top = input.racket_position - perceived_half_height;
        let racket_bottom = input.racket_position + perceived_half_height;

        // Decide whether the ball is approaching this racket (works for either side)
        let ball_approaching =
            (input.ball_velocity.x > 0.0 && input.racket_x > input.ball_position.x) || (input.ball_velocity.x < 0.0 && input.racket_x < input.ball_position.x);

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
            let deadzone = perceived_half_height;
            if input.racket_position < center_y - deadzone {
                RacketAction::MoveDown
            } else if input.racket_position > center_y + deadzone {
                RacketAction::MoveUp
            } else {
                RacketAction::Stay
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ControllerInput, PredictiveBehavior};
    use ggez::glam::Vec2;
    use std::collections::HashSet;

    fn base_input() -> ControllerInput {
        ControllerInput {
            ball_position: Vec2::new(100.0, 100.0),
            ball_velocity: Vec2::new(200.0, 50.0),
            racket_position: 200.0,
            racket_x: 600.0,
            screen_height: 400.0,
            pressed_keys: HashSet::new(),
        }
    }

    #[test]
    fn predictive_handles_simple_projection() {
        let predictive_behavior = PredictiveBehavior::new();
        let input = base_input();
        let y = predictive_behavior.predict_ball_y(&input);
        // sanity: should be within bounds
        assert!(y >= 0.0 && y <= input.screen_height);
    }

    #[test]
    fn predictive_handles_vertical_wrap() {
        let predictive_behavior = PredictiveBehavior::new();
        let mut input = base_input();
        input.ball_velocity = Vec2::new(50.0, 500.0);
        let y = predictive_behavior.predict_ball_y(&input);
        assert!(y >= 0.0 && y <= input.screen_height);
    }
}
