// Menu Navigation Controls:
// - Left/Right Arrow or TAB: Switch between Player 1 and Player 2
// - Up/Down Arrow: Cycle through player types for selected player
// - Number keys (1-4): Directly select player type (1=Human, 2=Easy, 3=Medium, 4=Hard)
// - Mouse Click: Select and cycle player type
// - SPACE/ENTER: Start game

use crate::ui::menu as ui_menu;
use crate::{audio::play_embedded_sound, ball::*, controller::ControllerInput, debug::DebugInfo, physics::*, player_type::PlayerType, racket::*, score::Score};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::{Context, GameResult, event, glam::Vec2, input::keyboard::KeyCode};
use std::collections::HashSet;

const MIDDLE_LINE_WIDTH: f32 = RACKET_WIDTH / 4.0;
const WINNING_SCORE: u8 = 10;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver { winner: Player },
}

pub struct MainState {
    state: GameState,
    player_left: Racket,
    player_right: Racket,
    player_left_type: PlayerType,
    player_right_type: PlayerType,
    selected_player: u8, // 1 for left, 2 for right (for keyboard navigation)
    ball: Ball,
    middle_line_mesh: Mesh,
    score: Score,
    debug: DebugInfo,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let player_left_type = PlayerType::Human;
        let player_right_type = PlayerType::AIHard;

        let left_controller = player_left_type.create_controller_for_player(Player::Left);
        let right_controller = player_right_type.create_controller_for_player(Player::Right);
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
            state: GameState::Menu,
            player_left: Racket::new(RACKET_OFFSET, screen_height_center, context, left_controller)?,
            player_right: Racket::new(screen_width - RACKET_OFFSET, screen_height_center, context, right_controller)?,
            player_left_type,
            player_right_type,
            selected_player: 1, // Start with player 1 selected
            ball: Ball::new(screen_width_center, screen_height_center, context)?,
            middle_line_mesh,
            score,
            debug: DebugInfo::new(),
        })
    }

    fn draw_centered_title(&self, canvas: &mut Canvas, context: &mut Context, text: &str, color: Color) -> GameResult {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let mut title = Text::new(text);
        title.set_scale(screen_height / 10.0);
        let title_dimensions = title.measure(context)?;
        let title_position = Vec2::new((screen_width - title_dimensions.x) / 2.0, screen_height / 3.0);
        canvas.draw(&title, DrawParam::default().dest(title_position).color(color));
        Ok(())
    }

    fn update_controllers(&mut self, context: &mut Context) -> GameResult {
        let screen_height = context.gfx.drawable_size().1;
        let screen_height_center = screen_height / 2.0;

        // Recreate rackets with new controllers
        self.player_left = Racket::new(
            RACKET_OFFSET,
            screen_height_center,
            context,
            self.player_left_type.create_controller_for_player(Player::Left),
        )?;

        self.player_right = Racket::new(
            context.gfx.drawable_size().0 - RACKET_OFFSET,
            screen_height_center,
            context,
            self.player_right_type.create_controller_for_player(Player::Right),
        )?;

        Ok(())
    }

    fn reset_game(&mut self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) = context.gfx.drawable_size();
        let (screen_width_center, screen_height_center) = (screen_width / 2.0, screen_height / 2.0);

        self.ball.reset(screen_width_center, screen_height_center);
        self.player_left.position_y = screen_height_center;
        self.player_right.position_y = screen_height_center;
        self.score = Score::new(context)?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let delta_time = context.time.delta().as_secs_f32();
        self.debug.update(context)?;

        // F1 toggles debug in all states
        if context.keyboard.is_key_just_pressed(KeyCode::F1) {
            self.debug.toggle();
        }

        match self.state {
            GameState::Menu => {
                // Keyboard navigation: Left/Right arrows to switch between players
                if context.keyboard.is_key_just_pressed(KeyCode::Left) {
                    self.selected_player = 1;
                } else if context.keyboard.is_key_just_pressed(KeyCode::Right) {
                    self.selected_player = 2;
                } else if context.keyboard.is_key_just_pressed(KeyCode::Tab) {
                    // Tab to toggle between players
                    self.selected_player = if self.selected_player == 1 { 2 } else { 1 };
                }

                // Up/Down arrows or number keys to change player type
                if context.keyboard.is_key_just_pressed(KeyCode::Up) {
                    if self.selected_player == 1 {
                        self.player_left_type = self.player_left_type.next();
                        self.update_controllers(context)?;
                    } else {
                        self.player_right_type = self.player_right_type.next();
                        self.update_controllers(context)?;
                    }
                } else if context.keyboard.is_key_just_pressed(KeyCode::Down) {
                    if self.selected_player == 1 {
                        self.player_left_type = self.player_left_type.previous();
                        self.update_controllers(context)?;
                    } else {
                        self.player_right_type = self.player_right_type.previous();
                        self.update_controllers(context)?;
                    }
                }

                // Number keys for direct selection (1=Human, 2=Easy, 3=Medium, 4=Hard)
                let selected_type = if context.keyboard.is_key_just_pressed(KeyCode::Key1) || context.keyboard.is_key_just_pressed(KeyCode::Numpad1) {
                    Some(PlayerType::Human)
                } else if context.keyboard.is_key_just_pressed(KeyCode::Key2) || context.keyboard.is_key_just_pressed(KeyCode::Numpad2) {
                    Some(PlayerType::AIEasy)
                } else if context.keyboard.is_key_just_pressed(KeyCode::Key3) || context.keyboard.is_key_just_pressed(KeyCode::Numpad3) {
                    Some(PlayerType::AIMedium)
                } else if context.keyboard.is_key_just_pressed(KeyCode::Key4) || context.keyboard.is_key_just_pressed(KeyCode::Numpad4) {
                    Some(PlayerType::AIHard)
                } else {
                    None
                };

                if let Some(new_type) = selected_type {
                    if self.selected_player == 1 {
                        self.player_left_type = new_type;
                        self.update_controllers(context)?;
                    } else {
                        self.player_right_type = new_type;
                        self.update_controllers(context)?;
                    }
                }

                // Handle mouse clicks for player type selection
                if context.mouse.button_just_pressed(ggez::event::MouseButton::Left) {
                    let mouse_position = context.mouse.position();
                    let mouse_vec = Vec2::new(mouse_position.x, mouse_position.y);
                    if let Some(clicked_player) = ui_menu::hit_test_player(context, mouse_vec) {
                        self.selected_player = clicked_player; // Update selected player
                        match clicked_player {
                            1 => {
                                self.player_left_type = self.player_left_type.next();
                                self.update_controllers(context)?;
                            }
                            2 => {
                                self.player_right_type = self.player_right_type.next();
                                self.update_controllers(context)?;
                            }
                            _ => {}
                        }
                    }
                }

                // Press SPACE or ENTER to start the game
                if context.keyboard.is_key_just_pressed(KeyCode::Space) || context.keyboard.is_key_just_pressed(KeyCode::Return) {
                    self.reset_game(context)?;
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Toggle pause with P
                if context.keyboard.is_key_just_pressed(KeyCode::P) {
                    self.state = GameState::Paused;
                    return Ok(());
                }
                self.update_playing(context, delta_time)?;
            }
            GameState::Paused => {
                // P resumes from pause
                if context.keyboard.is_key_just_pressed(KeyCode::P) {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver { .. } => {
                // Press SPACE or ENTER to return to menu
                if context.keyboard.is_key_just_pressed(KeyCode::Space) || context.keyboard.is_key_just_pressed(KeyCode::Return) {
                    self.state = GameState::Menu;
                }
                // Press R to restart round immediately
                if context.keyboard.is_key_just_pressed(KeyCode::R) {
                    self.reset_game(context)?;
                    self.state = GameState::Playing;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(context, Color::BLACK);

        match &self.state {
            GameState::Menu => {
                ui_menu::draw_menu(context, &mut canvas, self.player_left_type, self.player_right_type, self.selected_player)?;
            }
            GameState::Playing => {
                self.draw_playing(&mut canvas);
            }
            GameState::Paused => {
                self.draw_paused(context, &mut canvas)?;
            }
            GameState::GameOver { winner } => {
                self.draw_game_over(context, &mut canvas, *winner)?;
            }
        }

        self.debug.draw(&mut canvas);
        canvas.finish(context)?;
        Ok(())
    }
}

impl MainState {
    fn update_playing(&mut self, context: &mut Context, delta_time: f32) -> GameResult {
        self.debug.set_ball_info(context, self.ball.position, self.ball.velocity, self.ball.speed)?;

        // Move rackets (player 1: W/S, player 2: Up/Down)
        let mut pressed = HashSet::new();
        for k in context.keyboard.pressed_keys() {
            pressed.insert(*k);
        }

        let input_left = ControllerInput {
            ball_position: self.ball.position,
            ball_velocity: self.ball.velocity,
            racket_position: self.player_left.position_y,
            racket_x: self.player_left.position_x,
            screen_height: context.gfx.drawable_size().1,
            pressed_keys: pressed.clone(),
        };

        let input_right = ControllerInput {
            ball_position: self.ball.position,
            ball_velocity: self.ball.velocity,
            racket_position: self.player_right.position_y,
            racket_x: self.player_right.position_x,
            screen_height: context.gfx.drawable_size().1,
            pressed_keys: pressed,
        };

        self.player_left.update(&input_left, delta_time);
        self.player_right.update(&input_right, delta_time);

        if bounce_borders(&mut self.ball, context.gfx.drawable_size().1) {
            let _ = play_embedded_sound(context, "wall_bounce.wav");
        }

        if racket_collision(&mut self.ball, &self.player_left) || racket_collision(&mut self.ball, &self.player_right) {
            let _ = play_embedded_sound(context, "racket_hit.wav");
        }

        if let Some(scored) = check_score(&self.ball, context.gfx.drawable_size().0) {
            let _ = play_embedded_sound(context, "score.wav");
            match scored {
                Player::Left => {
                    self.score.increment_p1(context)?;
                    // Check if player 1 won
                    if self.score.get_p1_score() >= WINNING_SCORE {
                        self.state = GameState::GameOver { winner: Player::Left };
                        return Ok(());
                    }
                }
                Player::Right => {
                    self.score.increment_p2(context)?;
                    // Check if player 2 won
                    if self.score.get_p2_score() >= WINNING_SCORE {
                        self.state = GameState::GameOver { winner: Player::Right };
                        return Ok(());
                    }
                }
            }
            self.ball.reset(context.gfx.drawable_size().0 / 2.0, context.gfx.drawable_size().1 / 2.0);
        }

        self.ball.move_ball(delta_time);

        Ok(())
    }

    fn draw_playing(&self, canvas: &mut Canvas) {
        self.score.draw_on_canvas(canvas);
        canvas.draw(&self.middle_line_mesh, DrawParam::default());
        self.player_left.draw_on_canvas(canvas);
        self.player_right.draw_on_canvas(canvas);
        self.ball.draw_on_canvas(canvas);
    }

    fn draw_game_over(&self, context: &mut Context, canvas: &mut Canvas, winner: Player) -> GameResult {
        // First draw the game state
        self.draw_playing(canvas);

        // Semi-transparent overlay
        let overlay_rect = Rect::new(0.0, 0.0, context.gfx.drawable_size().0, context.gfx.drawable_size().1);
        let overlay_mesh = Mesh::new_rectangle(context, DrawMode::fill(), overlay_rect, Color::from_rgba(0, 0, 0, 180))?;
        canvas.draw(&overlay_mesh, DrawParam::default());

        let (screen_width, screen_height) = context.gfx.drawable_size();

        // Winner text
        let winner_text = match winner {
            Player::Left => "Player 1 Wins!",
            Player::Right => "Player 2 Wins!",
        };
        self.draw_centered_title(canvas, context, winner_text, Color::WHITE)?;

        // Press to continue
        let mut continue_text = Text::new("SPACE/ENTER: Menu   |   R: Restart   |   Esc: Quit");
        continue_text.set_scale(screen_height / 30.0);
        let continue_dimensions = continue_text.measure(context)?;
        let continue_position = Vec2::new((screen_width - continue_dimensions.x) / 2.0, screen_height * 0.65);
        canvas.draw(
            &continue_text,
            DrawParam::default().dest(continue_position).color(Color::from_rgb(200, 200, 200)),
        );

        Ok(())
    }

    fn draw_paused(&self, context: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Draw current game state in the background
        self.draw_playing(canvas);

        // Semi-transparent overlay
        let overlay_rect = Rect::new(0.0, 0.0, context.gfx.drawable_size().0, context.gfx.drawable_size().1);
        let overlay_mesh = Mesh::new_rectangle(context, DrawMode::fill(), overlay_rect, Color::from_rgba(0, 0, 0, 160))?;
        canvas.draw(&overlay_mesh, DrawParam::default());

        let (screen_width, screen_height) = context.gfx.drawable_size();

        // Paused title
        self.draw_centered_title(canvas, context, "Paused", Color::WHITE)?;

        // Hints
        let mut hint = Text::new("P: Resume   |   Esc: Quit");
        hint.set_scale(screen_height / 30.0);
        let hint_dimensions = hint.measure(context)?;
        let hint_position = Vec2::new((screen_width - hint_dimensions.x) / 2.0, screen_height * 0.65);
        canvas.draw(&hint, DrawParam::default().dest(hint_position).color(Color::from_rgb(200, 200, 200)));

        Ok(())
    }
}
