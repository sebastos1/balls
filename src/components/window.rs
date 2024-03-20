use crate::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub fn cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) && primary_window.cursor.grab_mode == CursorGrabMode::None {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}