#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Generating,
    Resolving,
    Paused,
    Solved,
}
