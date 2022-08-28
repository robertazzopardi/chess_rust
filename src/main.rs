mod board;
mod piece;

use bevy::{input::mouse::MouseMotion, prelude::*, time::FixedTimestep, winit::WinitSettings};
use chess::{ASSET_PATH, BOARD_WIDTH, PIECE_SIZE, SQUARES};
use piece::{add_pieces, King, Pawn};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug, Component)]
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

impl From<Side> for f32 {
    fn from(side: Side) -> Self {
        match side {
            Side::White => -1.,
            Side::Black => 1.,
        }
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
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Board Background
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(&format!("{ASSET_PATH}/board/board.png")),
        ..Default::default()
    });
}

fn tmp(query: Query<&Pawn, With<Player>>) {
    // dbg!(4324);
    // for k in &query {
    //     dbg!(k.sprite.transform);
    // }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Players)
        // .insert_resource(WinitSettings::game())
        .add_startup_system(window_config)
        .add_startup_system(setup)
        .add_system(tmp)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //         // .with_system(check_for_collisions)
        //         // .with_system(move_paddle.before(check_for_collisions))
        //         .with_system(move_piece),
        //     // .with_system(apply_velocity.before(check_for_collisions))
        //     // .with_system(play_collision_sound.after(check_for_collisions)),
        // )
        .add_system(bevy::window::close_on_esc)
        .run();
}
