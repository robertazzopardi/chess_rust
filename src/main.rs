mod board;
mod piece;

use bevy::prelude::*;
use chess::{ASSET_PATH, BOARD_WIDTH, PIECE_SIZE, SQUARES};

const RENDER_SCALE: u32 = (BOARD_WIDTH / 2) - (PIECE_SIZE / 2);

#[derive(Component)]
struct Pawn;

fn window_config(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(chess::BOARD_WIDTH as f32, chess::BOARD_WIDTH as f32);
    window.set_resizable(false);
}

fn create_pawn(texture: Handle<Image>, x: f32, y: f32) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x * 100. - RENDER_SCALE as f32, y, 1.),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Board Background
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(&format!("{ASSET_PATH}/board/board.png")),
        ..Default::default()
    });

    // Pawns
    for i in 0..SQUARES {
        commands
            .spawn_bundle(create_pawn(
                asset_server.load(&format!("{ASSET_PATH}/pieces/black_pawn.png")),
                i as f32,
                250.,
            ))
            .insert(Pawn);

        commands
            .spawn_bundle(create_pawn(
                asset_server.load(&format!("{ASSET_PATH}/pieces/white_pawn.png")),
                i as f32,
                -250.,
            ))
            .insert(Pawn);
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(window_config)
        .run();
}
