mod board;
mod piece;

use bevy::prelude::*;

fn window_config(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(chess::BOARD_WIDTH, chess::BOARD_WIDTH);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(window_config)
        .run();
}
