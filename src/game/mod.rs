pub mod ball;
pub mod physics;
pub mod racket;
pub mod score;

// Re-export commonly used items at crate::game::* if desired by callers.
pub use ball::*;
pub use physics::*;
pub use racket::*;
pub use score::*;
