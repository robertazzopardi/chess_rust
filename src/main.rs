mod board;
mod piece;

use bevy::{prelude::*, time::FixedTimestep, window::PresentMode, winit::WinitSettings};
use chess::ASSET_PATH;
use piece::{add_pieces, handle_piece_movement, PieceType};

const FRAME_TIME: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct Dragging;

#[derive(Component)]
struct Board;

#[derive(Clone, Copy, Debug, Component, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl From<Side> for &str {
    fn from(side: Side) -> Self {
        match side {
            Side::White => "white",
            Side::Black => "black",
        }
    }
}

impl Side {
    pub fn offset(&self) -> f32 {
        match self {
            Side::White => -1.,
            Side::Black => 1.,
        }
    }

    pub fn dir(&self) -> f32 {
        self.offset() * -1.
    }
}

pub struct GameState {
    pub turn: Side,
}

impl GameState {
    pub fn change_side(&mut self) {
        if self.turn == Side::White {
            self.turn = Side::Black;
        } else {
            self.turn = Side::White;
        };
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self { turn: Side::White }
    }
}

#[derive(Component, Debug)]
pub struct Player;

struct Players;

impl Plugin for Players {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_players);
    }
}

fn add_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    let side = Side::White;
    commands.spawn().insert(Player).insert(side);
    add_pieces(&mut commands, &asset_server, side);

    let side = Side::Black;
    commands.spawn().insert(Player).insert(side);
    add_pieces(&mut commands, &asset_server, side);
}

fn window_config(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(chess::BOARD_WIDTH as f32, chess::BOARD_WIDTH as f32);
    window.set_resizable(false);
    window.set_present_mode(PresentMode::AutoVsync);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Board Texture
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(&format!("{ASSET_PATH}/board/board.png")),
        ..Default::default()
    });
}

fn tmp(query: Query<(&PieceType, &Side)>) {
    for (_piece, _side) in &query {
        // println!("{:?} {:?}", piece, side);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Players)
        .insert_resource(GameState::default())
        .insert_resource(WinitSettings::game())
        .add_startup_system(window_config)
        .add_startup_system(setup)
        .add_system(tmp)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FRAME_TIME as f64))
                // .with_system(check_for_collisions)
                // .with_system(move_paddle.before(check_for_collisions))
                .with_system(handle_piece_movement),
            // .with_system(apply_velocity.before(check_for_collisions))
            // .with_system(play_collision_sound.after(check_for_collisions)),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}
