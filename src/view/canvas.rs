use sdl2::{render::Canvas, video::Window, Sdl};

pub struct GameCanvas {
    pub canvas: Canvas<Window>,
}

impl GameCanvas {
    pub fn new(sdl_context: &Sdl, screen_size: (u32, u32)) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Maze", screen_size.0, screen_size.1)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self { canvas }
    }
}
