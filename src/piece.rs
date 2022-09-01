use crate::{Dragging, GameState, Side};
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
    pub piece_type: Piece,
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
            piece_type: Piece(PieceType::Pawn),
            sprite: create_piece(pawn_texture.clone(), i as f32, 250. * offset),
            side,
        });
    }

    // Rooks
    let rook_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Rook),
        sprite: create_piece(rook_texture.clone(), 0., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Rook),
        sprite: create_piece(rook_texture, 7., 350. * offset),
        side,
    });

    // Knights
    let knight_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Knight),
        sprite: create_piece(knight_texture.clone(), 1., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Knight),
        sprite: create_piece(knight_texture, 6., 350. * offset),
        side,
    });

    // Bishop
    let bishop_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png"));
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Bishop),
        sprite: create_piece(bishop_texture.clone(), 2., 350. * offset),
        side,
    });
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Bishop),
        sprite: create_piece(bishop_texture, 5., 350. * offset),
        side,
    });

    // Queen
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::Queen),
        sprite: create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_queen.png")),
            4.,
            350. * offset,
        ),
        side,
    });

    // King
    commands.spawn_bundle(PieceBundle {
        piece_type: Piece(PieceType::King),
        sprite: create_piece(
            asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_king.png")),
            3.,
            350. * offset,
        ),
        side,
    });
}

fn check_if_piece(
    window: &Window,
    query: &mut Query<(Entity, &mut Transform, &Piece), With<Piece>>,
) -> Option<Entity> {
    if let Some(Vec2 { x, y }) = window.cursor_position() {
        let mx = 700. - ((x / 100.).floor() * 100.);
        let my = 700. - ((y / 100.).floor() * 100.);

        // println!("{x} {y}");
        // println!("{mx} {my} mouse\n");

        for (entity, mut piece_transform, piece) in query.iter_mut() {
            let Vec3 { x, y, z } = piece_transform.translation;

            // println!("{x} {y} {piece_type:?} piece");

            let px = 350. - x;
            let py = 350. - y;

            if px == mx && py == my {
                // piece_transform.translation.x = 350.;
                // piece_transform.translation.y = 350.;
                // piece_transform.translation = mxy;
                return Some(entity);
            }
        }
    }

    None
}

fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec3 {
    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    // Apply camera transform
    *camera * norm

    // Alternatively:
    //camera.mul_vec3(norm)
}

pub fn handle_mouse_input(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    // mut query: Query<(&mut Transform, &Piece), With<Piece>>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform, &Piece), With<Piece>>,
        Query<&Transform, With<Camera>>,
        Query<(Entity, &Transform), With<Dragging>>,
    )>,
) {
    let window = windows.get_primary().unwrap();

    let normalized_mouse_coords = {
        let mut m = Vec3::default();
        if let Some(position) = window.cursor_position() {
            let camera = set.p1();
            let camera_transform = camera.single();
            m = window_to_world(position, window, camera_transform);
        }
        m
    };

    if mouse_input.just_released(MouseButton::Left) {
        if let Ok((entity, _)) = set.p2().get_single() {
            commands.entity(entity).remove::<Dragging>();
            dbg!("not dragging any more");
        }
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(piece_entity) = check_if_piece(window, &mut set.p0()) {
            // dbg!(piece);
            commands.entity(piece_entity).insert(Dragging);
        }
    }

    if let Ok(piece_dragging) = set.p2().get_single_mut() {
        dbg!(piece_dragging);
    }

    // let mxy = {
    //     let mut m = Vec3::default();
    //     if let Some(position) = window.cursor_position() {
    //         let camera = set.p1();
    //         let camera_transform = camera.single();
    //         m = window_to_world(position, window, camera_transform);
    //     }
    //     m
    // };

    // if mouse_input.pressed(MouseButton::Left) {
    //     if let Some(Vec2 { x, y }) = window.cursor_position() {
    //         let mx = 700. - ((x / 100.).floor() * 100.);
    //         let my = 700. - ((y / 100.).floor() * 100.);

    //         println!("{x} {y}");
    //         println!("{mx} {my} mouse\n");

    //         for (mut piece_transform, piece_type) in query.iter_mut() {
    //             let Vec3 { x, y, z } = piece_transform.translation;

    //             println!("{x} {y} {piece_type:?} piece");

    //             let px = 350. - x;
    //             let py = 350. - y;

    //             if px == mx && py == my {
    //                 // println!("{px} {py} piece\n");
    //                 // piece_transform.translation.y += PIECE_SIZE as f32;
    //                 println!("{} {}", mx, my);
    //                 piece_transform.translation.x = 350.;
    //                 piece_transform.translation.y = 350.;
    //                 break;
    //             }
    //         }
    //     }
    // }
}
