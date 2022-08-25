use chess::{ASSET_PATH, PIECE_SIZE};

pub const EMPTY: Empty = Empty;

#[derive(Clone, Copy)]
pub enum Team {
    White,
    Black,
}

impl Team {
    fn parse_pawn_row(&self) -> i32 {
        match self {
            Team::White => 600,
            Team::Black => 200,
        }
    }
}

impl From<Team> for &str {
    fn from(team: Team) -> Self {
        match team {
            Team::White => "white",
            Team::Black => "black",
        }
    }
}

pub trait Logic {
    fn can_move(&self) -> bool;
}

pub struct Piece {
    team: Team,
    // pub texture: Texture<'static>,
    // pub rect: Rect,
}

impl Piece {
    fn new<'a>(team: Team, team_name: &str, x: i32, y: i32) -> Self {
        // let texture = texture_creator
        // .load_texture(format!("{ASSET_PATH}/pieces/{team_name}_pawn.svg"))
        // .unwrap();
        // let rect = Rect::new(x, y, PIECE_SIZE, PIECE_SIZE);
        Self {
            team,
            // texture,
            // rect,
        }
    }
}

// impl<'a> Drawable<'a> for Piece<'a> {
//     fn draw(&self, canvas: &mut Canvas<Window>) {
//         render_texture(canvas, &self.texture, self.rect);
//     }
// }

pub struct Empty;

impl Logic for Empty {
    fn can_move(&self) -> bool {
        false
    }
}

pub struct Pawn(pub Piece);

impl Pawn {
    pub fn new<'a>(team: Team, x: i32) -> Self {
        Self(Piece::new(team, team.into(), x, team.parse_pawn_row()))
    }
}

impl Logic for Pawn {
    fn can_move(&self) -> bool {
        todo!()
    }
}

pub struct Rook(Piece);

impl Logic for Rook {
    fn can_move(&self) -> bool {
        todo!()
    }
}

pub struct Knight(Piece);

impl Logic for Knight {
    fn can_move(&self) -> bool {
        todo!()
    }
}

pub struct Bishop(Piece);

impl Logic for Bishop {
    fn can_move(&self) -> bool {
        todo!()
    }
}

pub struct Queen(Piece);

impl Logic for Queen {
    fn can_move(&self) -> bool {
        todo!()
    }
}

pub struct King(Piece);

impl Logic for King {
    fn can_move(&self) -> bool {
        todo!()
    }
}
