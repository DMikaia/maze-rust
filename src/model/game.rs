use crate::view::board::Renderer;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};

pub struct Game {
    pub canvas: Canvas<Window>,
    pub event_queue: EventPump,
    pub cell: u32,
}

impl Game {
    pub fn new(
        cell: u32,
        sdl_context: &sdl2::Sdl,
        screen_size: (u32, u32),
    ) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Maze", screen_size.0, screen_size.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let renderer = Renderer::new(
            Color::RGB(216, 200, 150),
            Rect::new(0, 0, screen_size.0, screen_size.1),
        );

        let event_queue = sdl_context.event_pump()?;

        Ok(Game {
            cell,
            canvas,
            event_queue,
        })
    }
}
