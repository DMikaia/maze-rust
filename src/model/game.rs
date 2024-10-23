use sdl2::{render::Canvas, video::Window, EventPump};

pub struct Game {
    pub canvas: Canvas<Window>,
    pub event_queue: EventPump,
}

impl Game {
    pub fn new(sdl_context: &sdl2::Sdl, screen_size: (u32, u32)) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Maze", screen_size.0, screen_size.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        // let renderer = Renderer::new(
        //     Rect::new(0, 0, screen_size.0, screen_size.1),
        //     Color::RGB(26, 171, 252),
        //     &texture_loader,
        // );

        let event_queue = sdl_context.event_pump()?;

        Ok(Game {
            canvas,
            event_queue,
        })
    }
}
