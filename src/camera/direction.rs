//! Camera movement directions

/// Directions in which to move
#[derive(Debug)]
pub enum MoveDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

/// Directions to rotate in
#[derive(Debug)]
pub enum RotationDirection {
    Left,
    Right,
}
