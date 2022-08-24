use crate::BOARD_WIDTH;
use sdl2::{
    image::InitFlag,
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
    EventPump,
};
use sdl2::{render::Canvas, video::Window};

pub struct Renderer {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Could not init sdl");
        let video_subsystem = sdl_context
            .video()
            .expect("Could not create video subsystem");
        let _image_context = sdl2::image::init(InitFlag::PNG).expect("Could not init sdl_image");

        let window = video_subsystem
            .window("Chess", BOARD_WIDTH, BOARD_WIDTH)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .expect("Could not build window");

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .expect("Could not create canvas");

        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_context.event_pump().expect("Could not get event pump");

        Self {
            canvas,
            event_pump,
            texture_creator,
        }
    }
}

#[inline]
pub fn render_texture(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    canvas
        .copy_ex(
            texture,
            None,
            Some(Rect::new(x as i32, y as i32, w, h)),
            0.,
            None,
            false,
            false,
        )
        .expect("Could not render texture");
}
