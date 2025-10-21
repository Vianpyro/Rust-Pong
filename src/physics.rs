use crate::ball::{BALL_SIZE, BALL_SPEED_INCREMENT, BALL_SPEED_MAX, Ball};
use crate::racket::{RACKET_HEIGHT_HALF, RACKET_WIDTH_HALF, Racket};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Left,
    Right,
}

/// Bounce the ball off the top/bottom walls if needed.
pub fn bounce_borders(ball: &mut Ball, screen_h: f32) -> bool {
    if ball.position.y - BALL_SIZE / 2.0 <= 0.0 && ball.velocity.y < 0.0 || ball.position.y + BALL_SIZE / 2.0 >= screen_h && ball.velocity.y > 0.0 {
        ball.velocity.y = -ball.velocity.y;
        true
    } else {
        false
    }
}

pub fn racket_collision(ball: &mut Ball, racket: &Racket) -> bool {
    // Generalized collision: determine the ball contact x (edge) and the racket edge to compare against
    let contact_x = if ball.velocity.x < 0.0 {
        ball.position.x - BALL_SIZE / 2.0
    } else {
        ball.position.x + BALL_SIZE / 2.0
    };

    let racket_edge = if ball.velocity.x < 0.0 {
        racket.position_x + RACKET_WIDTH_HALF
    } else {
        racket.position_x - RACKET_WIDTH_HALF
    };

    let horizontal_overlap = if ball.velocity.x < 0.0 {
        contact_x <= racket_edge
    } else {
        contact_x >= racket_edge
    };

    let vertical_overlap = ball.position.y >= racket.position_y - RACKET_HEIGHT_HALF && ball.position.y <= racket.position_y + RACKET_HEIGHT_HALF;

    // Only reflect if ball is actually approaching the racket (prevents accidental reflections)
    let approaching = (ball.velocity.x < 0.0 && ball.position.x > racket.position_x) || (ball.velocity.x > 0.0 && ball.position.x < racket.position_x);

    if horizontal_overlap && vertical_overlap && approaching {
        ball.velocity.x = -ball.velocity.x;
        let offset = (ball.position.y - racket.position_y) / RACKET_HEIGHT_HALF;
        ball.velocity.y = ball.speed * offset;
        ball.speed = (ball.speed * BALL_SPEED_INCREMENT).min(BALL_SPEED_MAX);
        ball.velocity = ball.velocity.normalize() * ball.speed;
        return true;
    }

    false
}

pub fn check_score(ball: &Ball, screen_w: f32) -> Option<Player> {
    if ball.position.x < 0.0 {
        return Some(Player::Right);
    }
    if ball.position.x > screen_w {
        return Some(Player::Left);
    }
    None
}
