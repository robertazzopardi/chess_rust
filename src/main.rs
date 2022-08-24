use crate::renderer::Renderer;
use board::Board;
use renderer::render_texture;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, image::LoadTexture};
use std::time::Duration;

mod board;
mod piece;
mod renderer;

pub const SQUARES: u32 = 8;
pub const PIECE_SIZE: u32 = 100;
pub const BOARD_WIDTH: u32 = SQUARES * PIECE_SIZE;

// struct Game {}

fn main() -> Result<(), String> {
    let Renderer {
        mut canvas,
        mut event_pump,
        texture_creator,
    } = Renderer::new();

    // let board_texture = texture_creator
    //     .load_texture("assets/864630-chess/svg/board/board.svg")
    //     .unwrap();

    let board = Board::new(&texture_creator);

    // let white_pawn = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_pawn.svg",
    // );
    // let black_pawn = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_pawn.svg",
    // );

    // let white_rook = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_rook.svg",
    // );
    // let black_rook = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_rook.svg",
    // );

    // let white_knight = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_knight.svg",
    // );
    // let black_knight = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_knight.svg",
    // );

    // let white_bishop = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_bishop.svg",
    // );
    // let black_bishop = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_bishop.svg",
    // );

    // let white_queen = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_queen.svg",
    // );
    // let black_queen = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_queen.svg",
    // );

    // let white_king = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/white_king.svg",
    // );
    // let black_king = create_texture(
    //     &renderer.texture_creator,
    //     "assets/864630-chess/svg/pieces/black_king.svg",
    // );

    canvas.clear();
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        render_texture(&mut canvas, &board.texture, 0, 0, BOARD_WIDTH, BOARD_WIDTH);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        // The rest of the game loop goes here...
    }

    Ok(())
}
