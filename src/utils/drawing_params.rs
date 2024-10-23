#[derive(Clone, Copy, Debug)]
pub struct DrawingParams {
    pub cell: (i32, i32),
    pub screen: (i32, i32),
}

impl DrawingParams {
    pub fn new(cell: (i32, i32), screen: (i32, i32)) -> Self {
        Self { cell, screen }
    }
}
