use sdl2::render::Texture;

pub trait Logic {
    fn can_move(&self) -> bool;
    // fn asset_path<'a>() -> &'a str;
}

struct Piece<'a> {
    team: bool,
    texture: Texture<'a>,
}

struct Pawn<'a>(Piece<'a>);

impl Pawn<'_> {
    fn new(team: bool) -> Self {
        Self(Piece {
            team,
            texture: todo!(),
        })
    }
}

impl Logic for Pawn<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}

struct Rook<'a>(Piece<'a>);

impl Logic for Rook<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}

struct Knight<'a>(Piece<'a>);

impl Logic for Knight<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}

struct Bishop<'a>(Piece<'a>);

impl Logic for Bishop<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}

struct Queen<'a>(Piece<'a>);

impl Logic for Queen<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}

struct King<'a>(Piece<'a>);

impl Logic for King<'_> {
    fn can_move(&self) -> bool {
        todo!()
    }
}
