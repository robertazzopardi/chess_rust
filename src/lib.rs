use bevy::prelude::*;

pub const ASSET_PATH: &str = "864630-chess/svg";
pub const SQUARES: usize = 8;
pub const PIECE_SIZE: u32 = 100;
pub const BOARD_WIDTH: u32 = 800;
pub const RENDER_SCALE: u32 = (BOARD_WIDTH / 2) - (PIECE_SIZE / 2);

#[inline]
pub fn window_to_world(window: &Window, camera: &Transform) -> Vec3 {
    if let Some(position) = window.cursor_position() {
        let norm = Vec3::new(
            position.x - window.width() / 2.,
            position.y - window.height() / 2.,
            0.,
        );

        camera.mul_vec3(norm)
    } else {
        Vec3::default()
    }
}
