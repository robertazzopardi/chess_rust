use crate::{Dragging, Side};
use bevy::prelude::*;
use chess::{ASSET_PATH, RENDER_SCALE, SQUARES};

#[derive(Debug)]
pub struct Pawn;

#[derive(Debug)]
pub struct Rook;

#[derive(Debug)]
pub struct Knight;

#[derive(Debug)]
pub struct Bishop;

#[derive(Debug)]
pub struct Queen;

#[derive(Debug)]
pub struct King;

#[derive(Component, Debug)]
pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
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

macro_rules! spawn_piece {
    ($commands:expr, $texture:expr, $x:expr, $y:expr, $offset:expr, $side:expr, $piece:ident) => {
        $commands.spawn_bundle(PieceBundle {
            piece_type: Piece(PieceType::$piece($piece)),
            sprite: SpriteBundle {
                texture: $texture.clone(),
                transform: Transform {
                    translation: Vec3::new($x * 100. - RENDER_SCALE as f32, $y * $offset, 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            side: $side,
        })
    };
}

pub fn add_pieces(commands: &mut Commands, asset_server: &Res<AssetServer>, side: Side) {
    let color: &str = side.into();
    let offset: f32 = side.into();

    // Pawns
    let pawn_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_pawn.png"));
    for i in 0..SQUARES {
        spawn_piece!(commands, pawn_texture, i as f32, 250., offset, side, Pawn);
    }

    // Rooks
    let rook_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png"));
    spawn_piece!(commands, rook_texture, 0., 350., offset, side, Rook);
    spawn_piece!(commands, rook_texture, 7., 350., offset, side, Rook);

    // Knights
    let knight_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png"));
    spawn_piece!(commands, knight_texture, 1., 350., offset, side, Knight);
    spawn_piece!(commands, knight_texture, 6., 350., offset, side, Knight);

    // Bishop
    let bishop_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png"));
    spawn_piece!(commands, bishop_texture, 2., 350., offset, side, Bishop);
    spawn_piece!(commands, bishop_texture, 5., 350., offset, side, Bishop);

    // Queen
    let queen_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_queen.png"));
    spawn_piece!(commands, queen_texture, 4., 350., offset, side, Queen);

    // King
    let king_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_king.png"));
    spawn_piece!(commands, king_texture, 3., 350., offset, side, King);
}

fn check_if_piece(
    window: &Window,
    query: &mut Query<(Entity, &mut Transform), With<Piece>>,
) -> Option<Entity> {
    if let Some(Vec2 { x, y }) = window.cursor_position() {
        let mx = 700. - ((x / 100.).floor() * 100.);
        let my = 700. - ((y / 100.).floor() * 100.);

        for (entity, piece_transform) in query.iter_mut() {
            let Vec3 { x, y, .. } = piece_transform.translation;

            let px = 350. - x;
            let py = 350. - y;

            if px == mx && py == my {
                return Some(entity);
            }
        }
    }

    None
}

#[inline]
fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec3 {
    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    camera.mul_vec3(norm)
}

pub fn handle_piece_movement(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform), With<Piece>>,
        Query<&Transform, With<Camera>>,
        Query<(Entity, &mut Transform), With<Dragging>>,
    )>,
) {
    let window = windows.get_primary().unwrap();

    // Normalize camera Coordinates
    let normalized_mouse_coords = {
        let mut m = Vec3::default();
        if let Some(position) = window.cursor_position() {
            let camera = set.p1();
            let camera_transform = camera.single();
            m = window_to_world(position, window, camera_transform);
        }
        m
    };

    // Handle just releasing the mouse
    if mouse_input.just_released(MouseButton::Left) {
        if let Ok((entity, mut transform)) = set.p2().get_single_mut() {
            commands.entity(entity).remove::<Dragging>();
            transform.translation.z = 1.;
        }
    }

    // Handle pressing the mouse
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(piece_entity) = check_if_piece(window, &mut set.p0()) {
            commands.entity(piece_entity).insert(Dragging);
        }
    }

    // Drag piece if one is selected
    if let Ok((_, mut transform)) = set.p2().get_single_mut() {
        let Vec3 { x, y, .. } = normalized_mouse_coords;
        transform.translation.x = x;
        transform.translation.y = y;
        transform.translation.z = 32.;
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
