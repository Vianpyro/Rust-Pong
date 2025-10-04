use crate::ball::Ball;
use crate::ball::{BALL_SIZE, BALL_SPEED};
use crate::racket::Racket;

pub enum Player {
    Left,
    Right,
}

/// Bounce the ball off the top/bottom walls if needed.
pub fn bounce_borders(ball: &mut Ball, screen_h: f32) {
    if ball.position.y - BALL_SIZE / 2.0 <= 0.0 && ball.velocity.y < 0.0 || ball.position.y + BALL_SIZE / 2.0 >= screen_h && ball.velocity.y > 0.0 {
        ball.velocity.y = -ball.velocity.y;
    }
}

pub fn racket_collision(ball: &mut Ball, racket: &Racket) -> bool {
    // Left-side collision
    if ball.position.x - BALL_SIZE / 2.0 <= racket.pos_x + crate::racket::RACKET_WIDTH_HALF
        && ball.position.y >= racket.pos_y - crate::racket::RACKET_HEIGHT_HALF
        && ball.position.y <= racket.pos_y + crate::racket::RACKET_HEIGHT_HALF
        && ball.velocity.x < 0.0
    {
        ball.velocity.x = -ball.velocity.x;
        let offset = (ball.position.y - racket.pos_y) / crate::racket::RACKET_HEIGHT_HALF;
        ball.velocity.y = BALL_SPEED * offset;
        ball.velocity = ball.velocity.normalize() * BALL_SPEED;
        return true;
    }

    // Right-side collision
    if ball.position.x + BALL_SIZE / 2.0 >= racket.pos_x - crate::racket::RACKET_WIDTH_HALF
        && ball.position.y >= racket.pos_y - crate::racket::RACKET_HEIGHT_HALF
        && ball.position.y <= racket.pos_y + crate::racket::RACKET_HEIGHT_HALF
        && ball.velocity.x > 0.0
    {
        ball.velocity.x = -ball.velocity.x;
        let offset = (ball.position.y - racket.pos_y) / crate::racket::RACKET_HEIGHT_HALF;
        ball.velocity.y = BALL_SPEED * offset;
        ball.velocity = ball.velocity.normalize() * BALL_SPEED;
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
