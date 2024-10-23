use sdl2::{pixels::Color, rect::Rect};

pub struct Renderer {
    pub base_color: Color,
    pub screen_area: Rect,
}

impl Renderer {
    pub fn new(base_color: Color, screen_area: Rect) -> Self {
        Self {
            base_color,
            screen_area,
        }
    }
}
