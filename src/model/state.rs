#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Generating,
    Resolving,
    Displaying,
    Complete,
}
