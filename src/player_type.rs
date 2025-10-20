use crate::controller::{AIController, Controller, HumanController};
use ggez::input::keyboard::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerType {
    Human,
    AIEasy,
    AIMedium,
    AIHard,
}

impl PlayerType {
    pub fn next(&self) -> Self {
        match self {
            PlayerType::Human => PlayerType::AIEasy,
            PlayerType::AIEasy => PlayerType::AIMedium,
            PlayerType::AIMedium => PlayerType::AIHard,
            PlayerType::AIHard => PlayerType::Human,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            PlayerType::Human => PlayerType::AIHard,
            PlayerType::AIEasy => PlayerType::Human,
            PlayerType::AIMedium => PlayerType::AIEasy,
            PlayerType::AIHard => PlayerType::AIMedium,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            PlayerType::Human => "Human",
            PlayerType::AIEasy => "AI - Easy",
            PlayerType::AIMedium => "AI - Medium",
            PlayerType::AIHard => "AI - Hard",
        }
    }

    pub fn create_controller_for_player(&self, player: u8) -> Box<dyn Controller> {
        match self {
            PlayerType::Human => {
                if player == 1 {
                    Box::new(HumanController::new(KeyCode::W, KeyCode::S))
                } else {
                    Box::new(HumanController::new(KeyCode::Up, KeyCode::Down))
                }
            }
            PlayerType::AIEasy => Box::new(AIController::easy()),
            PlayerType::AIMedium => Box::new(AIController::medium()),
            PlayerType::AIHard => Box::new(AIController::hard()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerType;

    #[test]
    fn next_cycles_in_order() {
        assert_eq!(PlayerType::Human.next(), PlayerType::AIEasy);
        assert_eq!(PlayerType::AIEasy.next(), PlayerType::AIMedium);
        assert_eq!(PlayerType::AIMedium.next(), PlayerType::AIHard);
        assert_eq!(PlayerType::AIHard.next(), PlayerType::Human);
    }

    #[test]
    fn previous_cycles_in_order() {
        assert_eq!(PlayerType::Human.previous(), PlayerType::AIHard);
        assert_eq!(PlayerType::AIEasy.previous(), PlayerType::Human);
        assert_eq!(PlayerType::AIMedium.previous(), PlayerType::AIEasy);
        assert_eq!(PlayerType::AIHard.previous(), PlayerType::AIMedium);
    }
}
