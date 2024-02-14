//! This module contains the game models.

/// Tracks the game state.
#[derive(Clone)]
pub enum State {
    Playing,
    Paused,
    Over,
    Win,
}

/// Track player movement direction.
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

/// A coordinate on the game grid.
#[derive(Copy, Clone)]
pub struct Coordinate(pub i32, pub i32);

impl std::ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    /// Add two coordinates together.
    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::cmp::PartialEq for Coordinate {
    /// Compare two coordinates for equality.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
