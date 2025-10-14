use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;

pub trait Controller {
    fn get_action(&mut self, game_state: &GameState) -> RacketAction;
}

pub enum RacketAction {
    MoveUp,
    MoveDown,
    Stay,
}

pub struct GameState {
    pub ball_pos: Vec2,
    pub ball_vel: Vec2,
    pub racket_pos: f32,
    pub opponent_pos: f32,
    pub screen_height: f32,
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
    fn get_action(&mut self, _game_state: &GameState) -> RacketAction {
        RacketAction::Stay
    }
}

pub struct AIController {}

impl AIController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for AIController {
    fn get_action(&mut self, game_state: &GameState) -> RacketAction {
        if game_state.ball_pos.y < game_state.racket_pos {
            RacketAction::MoveUp
        } else if game_state.ball_pos.y > game_state.racket_pos {
            RacketAction::MoveDown
        } else {
            RacketAction::Stay
        }
    }
}
