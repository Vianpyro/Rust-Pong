use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::{Context, GameResult, glam::Vec2};

use crate::player_type::PlayerType;

// Layout ratios for the menu UI (tweak here to adjust spacing/size)
const BOX_WIDTH_RATIO: f32 = 0.35;
const BOX_HEIGHT_RATIO: f32 = 0.25;
const BOX_Y_RATIO: f32 = 0.4;
const LEFT_BOX_X_RATIO: f32 = 0.1;
const RIGHT_BOX_X_RATIO: f32 = 0.55;

// Public API for the menu UI module
// - MenuLayout: geometry for player boxes
// - draw_menu: renders the complete menu screen
// - hit_test_player: returns 1 or 2 if mouse is inside a player box

#[derive(Debug, Clone, Copy)]
pub struct MenuLayout {
    pub left_box: Rect,
    pub right_box: Rect,
}

impl MenuLayout {
    pub fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let box_width = screen_width * BOX_WIDTH_RATIO;
        let box_height = screen_height * BOX_HEIGHT_RATIO;
        let box_y = screen_height * BOX_Y_RATIO;
        let left_box_x = screen_width * LEFT_BOX_X_RATIO;
        let right_box_x = screen_width * RIGHT_BOX_X_RATIO;

        Self {
            left_box: Rect {
                x: left_box_x,
                y: box_y,
                w: box_width,
                h: box_height,
            },
            right_box: Rect {
                x: right_box_x,
                y: box_y,
                w: box_width,
                h: box_height,
            },
        }
    }

    fn contains(rect: Rect, p: Vec2) -> bool {
        p.x >= rect.x && p.x <= rect.x + rect.w && p.y >= rect.y && p.y <= rect.y + rect.h
    }
}

pub fn hit_test_player_with_layout(layout: &MenuLayout, mouse_position: Vec2) -> Option<u8> {
    if MenuLayout::contains(layout.left_box, mouse_position) {
        Some(1)
    } else if MenuLayout::contains(layout.right_box, mouse_position) {
        Some(2)
    } else {
        None
    }
}

pub fn hit_test_player(context: &Context, mouse_position: Vec2) -> Option<u8> {
    let (screen_width, screen_height) = context.gfx.drawable_size();
    let layout = MenuLayout::from_screen(screen_width, screen_height);
    hit_test_player_with_layout(&layout, mouse_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_test_left_and_right_boxes() {
        let layout = MenuLayout::from_screen(800.0, 600.0);

        let left_center = Vec2::new(layout.left_box.x + layout.left_box.w / 2.0, layout.left_box.y + layout.left_box.h / 2.0);
        let right_center = Vec2::new(layout.right_box.x + layout.right_box.w / 2.0, layout.right_box.y + layout.right_box.h / 2.0);

        assert_eq!(hit_test_player_with_layout(&layout, left_center), Some(1));
        assert_eq!(hit_test_player_with_layout(&layout, right_center), Some(2));
        assert_eq!(hit_test_player_with_layout(&layout, Vec2::new(0.0, 0.0)), None);
    }
}

pub fn draw_menu(context: &mut Context, canvas: &mut Canvas, left_type: PlayerType, right_type: PlayerType, selected_player: u8) -> GameResult {
    let (screen_width, screen_height) = context.gfx.drawable_size();
    let layout = MenuLayout::from_screen(screen_width, screen_height);

    // Title
    let mut title = Text::new("PONG");
    title.set_scale(screen_height / 6.0);
    let title_dimensions = title.measure(context)?;
    let title_position = Vec2::new((screen_width - title_dimensions.x) / 2.0, screen_height * 0.15);
    canvas.draw(&title, DrawParam::default().dest(title_position).color(Color::WHITE));

    // Player 1
    draw_player_box(context, canvas, layout.left_box, "Player 1", left_type, selected_player == 1)?;

    // Player 2
    draw_player_box(context, canvas, layout.right_box, "Player 2", right_type, selected_player == 2)?;

    // Keyboard instructions
    let mut keyboard_instructions = Text::new("← → or TAB: Select Player  |  ↑ ↓: Change Type  |  1-4: Direct Select");
    keyboard_instructions.set_scale(screen_height / 35.0);
    let keyboard_dimensions = keyboard_instructions.measure(context)?;
    let keyboard_position = Vec2::new((screen_width - keyboard_dimensions.x) / 2.0, screen_height * 0.72);
    canvas.draw(
        &keyboard_instructions,
        DrawParam::default().dest(keyboard_position).color(Color::from_rgb(150, 150, 150)),
    );

    // Mouse instructions
    let mut instructions = Text::new("Click on players to change type");
    instructions.set_scale(screen_height / 35.0);
    let instructions_dimensions = instructions.measure(context)?;
    let instructions_position = Vec2::new((screen_width - instructions_dimensions.x) / 2.0, screen_height * 0.78);
    canvas.draw(
        &instructions,
        DrawParam::default().dest(instructions_position).color(Color::from_rgb(150, 150, 150)),
    );

    // Start/quit instruction
    let mut start_text = Text::new("SPACE/ENTER: Start   |   Esc: Quit");
    start_text.set_scale(screen_height / 28.0);
    let start_dimensions = start_text.measure(context)?;
    let start_position = Vec2::new((screen_width - start_dimensions.x) / 2.0, screen_height * 0.85);
    canvas.draw(&start_text, DrawParam::default().dest(start_position).color(Color::from_rgb(200, 200, 200)));

    Ok(())
}

fn draw_player_box(context: &mut Context, canvas: &mut Canvas, rect: Rect, player_name: &str, player_type: PlayerType, is_selected: bool) -> GameResult {
    // Background
    let box_mesh = Mesh::new_rectangle(context, DrawMode::fill(), rect, ui_color_box_bg(is_selected))?;
    canvas.draw(&box_mesh, DrawParam::default());

    // Border
    let border_width = if is_selected { 5.0 } else { 3.0 };
    let border_mesh = Mesh::new_rectangle(context, DrawMode::stroke(border_width), rect, ui_color_box_border(is_selected))?;
    canvas.draw(&border_mesh, DrawParam::default());

    let (_, screen_height) = context.gfx.drawable_size();

    // Name
    let mut name_text = Text::new(player_name);
    name_text.set_scale(screen_height / 25.0);
    let name_dimensions = name_text.measure(context)?;
    let name_position = Vec2::new(rect.x + (rect.w - name_dimensions.x) / 2.0, rect.y + rect.h * 0.2);
    canvas.draw(&name_text, DrawParam::default().dest(name_position).color(Color::WHITE));

    // Type
    let mut type_text = Text::new(player_type.display_name());
    type_text.set_scale(screen_height / 20.0);
    let type_dimensions = type_text.measure(context)?;
    let type_position = Vec2::new(rect.x + (rect.w - type_dimensions.x) / 2.0, rect.y + rect.h * 0.55);
    canvas.draw(&type_text, DrawParam::default().dest(type_position).color(ui_color_player_type(player_type)));

    Ok(())
}

// Colors helpers for consistent styling
fn ui_color_box_bg(is_selected: bool) -> Color {
    if is_selected {
        Color::from_rgba(60, 60, 90, 255)
    } else {
        Color::from_rgba(40, 40, 60, 255)
    }
}

fn ui_color_box_border(is_selected: bool) -> Color {
    if is_selected {
        Color::from_rgb(150, 200, 255)
    } else {
        Color::from_rgb(100, 100, 150)
    }
}

fn ui_color_player_type(player_type: PlayerType) -> Color {
    match player_type {
        PlayerType::Human => Color::from_rgb(100, 200, 100),
        PlayerType::AIEasy => Color::from_rgb(100, 150, 255),
        PlayerType::AIMedium => Color::from_rgb(255, 200, 100),
        PlayerType::AIHard => Color::from_rgb(255, 100, 100),
    }
}
