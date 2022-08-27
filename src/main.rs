mod board;
mod piece;

use bevy::prelude::*;
use chess::{ASSET_PATH, BOARD_WIDTH, PIECE_SIZE, SQUARES};

const RENDER_SCALE: u32 = (BOARD_WIDTH / 2) - (PIECE_SIZE / 2);

#[derive(Clone, Copy, Debug)]
enum Side {
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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Team(Side);

fn add_players(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Team(Side::White));
    commands.spawn().insert(Player).insert(Team(Side::Black));
}

#[derive(Component)]
struct Pawn;

#[derive(Component)]
struct Rook;

#[derive(Component)]
struct Knight;

#[derive(Component)]
struct Bishop;

#[derive(Component)]
struct Queen;

#[derive(Component)]
struct King;

fn window_config(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(chess::BOARD_WIDTH as f32, chess::BOARD_WIDTH as f32);
    window.set_resizable(false);
}

fn create_piece(texture: Handle<Image>, x: f32, y: f32) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x * 100. - RENDER_SCALE as f32, y, 1.),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn add_pieces(commands: &mut Commands, asset_server: &Res<AssetServer>, team: Team) {
    let color: &str = team.0.into();
    let offset: f32 = team.0.into();

    // Pawns
    for i in 0..SQUARES {
        commands
            .spawn_bundle(create_piece(
                asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_pawn.png")),
                i as f32,
                250. * offset,
            ))
            .insert(Pawn);
    }

    // Rooks
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png")),
            0.,
            350. * offset,
        ))
        .insert(Rook);
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png")),
            7.,
            350. * offset,
        ))
        .insert(Rook);

    // Knights
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png")),
            1.,
            350. * offset,
        ))
        .insert(Knight);
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png")),
            6.,
            350. * offset,
        ))
        .insert(Knight);

    // Bishop
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png")),
            2.,
            350. * offset,
        ))
        .insert(Bishop);
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png")),
            5.,
            350. * offset,
        ))
        .insert(Bishop);

    // Queen
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_queen.png")),
            4.,
            350. * offset,
        ))
        .insert(Queen);

    //King
    commands
        .spawn_bundle(create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_king.png")),
            3.,
            350. * offset,
        ))
        .insert(King);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Board Background
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(&format!("{ASSET_PATH}/board/board.png")),
        ..Default::default()
    });

    add_pieces(&mut commands, &asset_server, Team(Side::White));
    add_pieces(&mut commands, &asset_server, Team(Side::Black));
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(add_players)
        .add_plugins(DefaultPlugins)
        .add_system(window_config)
        .run();
}
