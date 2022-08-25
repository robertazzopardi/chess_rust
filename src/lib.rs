use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

pub const ASSET_PATH: &str = "assets/864630-chess/svg";
pub const SQUARES: u32 = 8;
pub const PIECE_SIZE: u32 = 100;
pub const BOARD_WIDTH: f32 = (SQUARES * PIECE_SIZE) as f32;

#[derive(EnumCountMacro, EnumIter)]
pub enum TextureId {
    Board,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
}
