use crate::Side;
use bevy::prelude::*;
use chess::{ASSET_PATH, PIECE_SIZE, RENDER_SCALE, SQUARES};

#[derive(Component, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Component, Debug)]
pub struct Piece(PieceType);

#[derive(Component, Bundle)]
pub struct PieceBundle {
    pub piece_type: PieceType,
    pub side: Side,
    #[bundle]
    pub sprite: SpriteBundle,
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

pub fn add_pieces(commands: &mut Commands, asset_server: &Res<AssetServer>, side: Side) {
    let color: &str = side.into();
    let offset: f32 = side.into();

    // Pawns
    let pawn_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_pawn.png"));
    for i in 0..SQUARES {
        commands.spawn_bundle(PieceBundle {
            piece_type: PieceType::Pawn,
            sprite: create_piece(pawn_texture.clone(), i as f32, 250. * offset),
            side,
        });
    }

    // Rooks
    let rook_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Rook,
        sprite: create_piece(rook_texture.clone(), 0., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Rook,
        sprite: create_piece(rook_texture, 7., 350. * offset),
        side,
    });

    // Knights
    let knight_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Knight,
        sprite: create_piece(knight_texture.clone(), 1., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Knight,
        sprite: create_piece(knight_texture, 6., 350. * offset),
        side,
    });

    // Bishop
    let bishop_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Bishop,
        sprite: create_piece(bishop_texture.clone(), 2., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Bishop,
        sprite: create_piece(bishop_texture, 5., 350. * offset),
        side,
    });

    // Queen
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::Queen,
        sprite: create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_queen.png")),
            4.,
            350. * offset,
        ),
        side,
    });

    // King
    commands.spawn_bundle(PieceBundle {
        piece_type: PieceType::King,
        sprite: create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_king.png")),
            3.,
            350. * offset,
        ),
        side,
    });
}

pub fn move_piece(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Piece>>,
) {
    let window = windows.get_primary().unwrap();

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(Vec2 { x, y }) = window.cursor_position() {
            let mx = ((x / 100.).floor() * 100.) as u32;
            let my = ((y / 100.).floor() * 100.) as u32;

            println!("{mx} {my} mouse\n");

            for mut piece_transform in query.iter_mut() {
                // let mut direction = 0.0;

                let Vec3 { x, y, .. } = piece_transform.translation;
                let py = ((x / 100.).floor() * 100.) as u32;
                let px = ((y / 100.).floor() * 100.) as u32;

                // println!("{px} {py} piece\n");
                if px == mx && py == my {
                    piece_transform.translation.y += PIECE_SIZE as f32;
                }

                // if mouse_input.just_released(MouseButton::Left) {}

                // let new_paddle_position =
                //     piece_transform.translation.x + direction * 1. * TIME_STEP;

                // piece_transform.translation.x = new_paddle_position;
            }
        }
    }
}
