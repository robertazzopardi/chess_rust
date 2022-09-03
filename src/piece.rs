use crate::{Dragging, GameState, Side};
use bevy::prelude::*;
use chess::{window_to_world, ASSET_PATH, RENDER_SCALE, SQUARES};

pub trait Logic {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool;
}

#[derive(Debug, PartialEq)]
pub struct Pawn;

impl Logic for Pawn {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        let dir: f32 = side.dir();

        let Vec3 { x, y, .. } = new_pos - old_pos;

        if x == 0. && y == dir * 100. {
            return true;
        }
        if x == 0. && y == dir * 200. && made_first_move.is_none() {
            return true;
        }

        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Rook;

impl Logic for Rook {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Knight;

impl Logic for Knight {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Bishop;

impl Logic for Bishop {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Queen;

impl Logic for Queen {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct King;

impl Logic for King {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        false
    }
}

#[derive(Component, Debug, PartialEq)]
pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl Logic for PieceType {
    fn can_move(
        &self,
        side: Side,
        old_pos: Vec3,
        new_pos: Vec3,
        made_first_move: Option<&MadeFirstMove>,
    ) -> bool {
        match self {
            PieceType::Pawn(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
            PieceType::Rook(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
            PieceType::Knight(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
            PieceType::Bishop(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
            PieceType::Queen(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
            PieceType::King(inner) => inner.can_move(side, old_pos, new_pos, made_first_move),
        }
    }
}

#[derive(Component, Debug, Deref)]
pub struct Piece(PieceType);

#[derive(Component, Bundle)]
pub struct PieceBundle {
    pub piece_type: Piece,
    pub side: Side,
    #[bundle]
    pub sprite: SpriteBundle,
}

macro_rules! spawn_piece {
    ($commands:expr, $texture:expr,  $y:expr, $offset:expr, $side:expr, $piece:ident, $( $x:expr ),*) => {
        $(
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
            });
        )*
    };
}

pub fn add_pieces(commands: &mut Commands, asset_server: &Res<AssetServer>, side: Side) {
    let color: &str = side.into();
    let offset: f32 = side.offset();

    // Pawns
    let pawn_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_pawn.png"));
    for i in 0..SQUARES {
        spawn_piece!(commands, pawn_texture, 250., offset, side, Pawn, i as f32);
    }

    // Rooks
    let rook_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_rook.png"));
    spawn_piece!(commands, rook_texture, 350., offset, side, Rook, 0., 7.);

    // Knights
    let knight_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_knight.png"));
    spawn_piece!(commands, knight_texture, 350., offset, side, Knight, 1., 6.);

    // Bishop
    let bishop_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_bishop.png"));
    spawn_piece!(commands, bishop_texture, 350., offset, side, Bishop, 2., 5.);

    // Queen
    let queen_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_queen.png"));
    spawn_piece!(commands, queen_texture, 350., offset, side, Queen, 4.);

    // King
    let king_texture = asset_server.load(&format!("{ASSET_PATH}/pieces/{color}_king.png"));
    spawn_piece!(commands, king_texture, 350., offset, side, King, 3.);
}

fn check_if_piece(
    window: &Window,
    query: &mut Query<(Entity, &mut Transform), With<Piece>>,
) -> Option<(Entity, Vec3)> {
    if let Some(Vec2 { x, y }) = window.cursor_position() {
        let mx = 700. - ((x / 100.).floor() * 100.);
        let my = 700. - ((y / 100.).floor() * 100.);

        for (entity, piece_transform) in query.iter_mut() {
            let Vec3 { x, y, .. } = 350. - piece_transform.translation;

            // let px = 350. - x;
            // let py = 350. - y;

            if x == mx && y == my {
                return Some((entity, piece_transform.translation));
            }
        }
    }

    None
}

#[derive(Component)]
pub struct OldPosition(Vec3);

#[derive(Component)]
pub struct MadeFirstMove;

pub fn handle_piece_movement(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform), With<Piece>>,
        Query<&Transform, With<Camera>>,
        Query<
            (
                Entity,
                &mut Transform,
                &OldPosition,
                &Piece,
                &Side,
                Option<&MadeFirstMove>,
            ),
            With<Dragging>,
        >,
    )>,
) {
    let window = windows.primary();

    // Normalize camera Coordinates
    let normalized_mouse_coords = {
        let mut m = Vec3::default();
        if let Some(position) = window.cursor_position() {
            m = window_to_world(position, window, set.p1().single());
        }
        m
    };

    // Handle just releasing the mouse
    if mouse_input.just_released(MouseButton::Left) {
        if let Ok(piece_entity) = set.p2().get_single_mut() {
            let aligned_mouse_coords = (normalized_mouse_coords / 50.).round() * 50.;

            let (entity, mut transform, old_pos, piece, side, made_first_move) = piece_entity;

            if *side == game_state.turn
                && piece.can_move(*side, old_pos.0, aligned_mouse_coords, made_first_move)
            {
                transform.translation = aligned_mouse_coords;
                transform.translation.z = 1.;

                if piece.0 == PieceType::Pawn(Pawn) && made_first_move.is_none() {
                    commands.entity(entity).insert(MadeFirstMove);
                }

                game_state.change_side();
            } else {
                transform.translation = old_pos.0;
            }

            commands
                .entity(entity)
                .remove::<Dragging>()
                .remove::<OldPosition>();
        }
    }

    // Handle pressing the mouse
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some((piece_entity, translation)) = check_if_piece(window, &mut set.p0()) {
            commands
                .entity(piece_entity)
                .insert(Dragging)
                .insert(OldPosition(translation));
        }
    }

    // Drag piece if one is selected
    if let Ok((_, mut transform, _, piece, _, _)) = set.p2().get_single_mut() {
        if !mouse_input.just_released(MouseButton::Left) {
            transform.translation = normalized_mouse_coords;
            transform.translation.z = 32.;
        }
    }
}
