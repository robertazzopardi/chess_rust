use crate::{piece::Logic, SQUARES};
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct Board<'a> {
    pub grid: [[Option<Box<dyn Logic>>; SQUARES as usize]; SQUARES as usize],
    // pub grid: Vec<Vec<Box<dyn Logic>>>,
    pub texture: Texture<'a>,
}

impl<'a> Board<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let grid: [[Option<Box<dyn Logic>>; SQUARES as usize]; SQUARES as usize] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        let texture = texture_creator
            .load_texture("assets/864630-chess/svg/board/board.svg")
            .unwrap();

        Self { grid, texture }
    }
}
