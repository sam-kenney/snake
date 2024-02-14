//! Game logic for snake.
use crate::models::{Coordinate, Direction, State};
use rand::Rng;
use std::time::Duration;

/// Store data about the game.
pub struct Game {
    pub player_position: Vec<Coordinate>,
    pub player_direction: Direction,
    pub food: Coordinate,
    pub state: State,
}

impl Game {
    /// Create a new game with a random food position.
    pub fn new() -> Game {
        let mut game = Game {
            player_position: vec![Coordinate(3, 1), Coordinate(2, 1), Coordinate(1, 1)],
            player_direction: Direction::Right,
            state: State::Paused,
            food: Coordinate(5, 5),
        };

        game.food = game.generate_food();
        game
    }

    /// Process the next tick of the game.
    /// Moves the snake, checks for collisions, and updates the game state.
    pub fn next_tick(&mut self) {
        if let State::Win | State::Over = self.state {
            self.reset();
        }

        if let State::Paused = self.state {
            return;
        }

        let head_position = self.player_position.first().unwrap();

        let next_head_position = match self.player_direction {
            Direction::Up => *head_position + Coordinate(0, -1),
            Direction::Down => *head_position + Coordinate(0, 1),
            Direction::Right => *head_position + Coordinate(1, 0),
            Direction::Left => *head_position + Coordinate(-1, 0),
        };

        if self.collision(next_head_position) {
            self.state = State::Over;
            return;
        }

        if self.ate_food() {
            if self.player_position.len() == crate::SNAKE_SIZE_FOR_WIN {
                self.state = State::Win;
            }
            self.food = self.generate_food();
        } else {
            self.player_position.pop();
        }

        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    /// Generate a new food position that doesn't collide with the snake.
    fn generate_food(&mut self) -> Coordinate {
        loop {
            let x = rand::thread_rng().gen_range(0..crate::GRID_X_SIZE as i32);
            let y = rand::thread_rng().gen_range(0..crate::GRID_Y_SIZE as i32);
            let new_food = Coordinate(x, y);
            if !self.player_position.contains(&new_food) {
                break new_food;
            }
        }
    }

    /// Check if the next head position is a collision with the snake or the arena.
    fn collision(&mut self, next_head_position: Coordinate) -> bool {
        // Player has exited arena.
        if next_head_position.0 < 0
            || next_head_position.1 < 0
            || next_head_position.0 >= crate::GRID_X_SIZE as i32
            || next_head_position.1 >= crate::GRID_Y_SIZE as i32
        {
            return true;
        }

        // Player has hit self.
        if self.player_position.contains(&next_head_position) {
            return true;
        }

        false
    }

    /// Reset the game to its initial state.
    pub fn reset(&mut self) {
        std::thread::sleep(Duration::new(0, 200000000));
        self.player_position = vec![Coordinate(3, 1), Coordinate(2, 1), Coordinate(1, 1)];
        self.player_direction = Direction::Right;
        self.state = State::Paused;
        self.food = self.generate_food();
    }

    /// Move the snake up.
    pub fn move_up(&mut self) {
        // Don't let the player double back.
        if let Direction::Down = self.player_direction {
            return;
        };
        self.player_direction = Direction::Up;
    }

    /// Move the snake down.
    pub fn move_down(&mut self) {
        // Don't let the player double back.
        if let Direction::Up = self.player_direction {
            return;
        };
        self.player_direction = Direction::Down;
    }

    /// Move the snake right.
    pub fn move_right(&mut self) {
        // Don't let the player double back.
        if let Direction::Left = self.player_direction {
            return;
        };
        self.player_direction = Direction::Right;
    }

    /// Move the snake left.
    pub fn move_left(&mut self) {
        // Don't let the player double back.
        if let Direction::Right = self.player_direction {
            return;
        };
        self.player_direction = Direction::Left;
    }

    /// Check if the snake's head collided with the food.
    pub fn ate_food(&mut self) -> bool {
        *self.player_position.first().unwrap() == self.food
    }

    /// Toggle the game state between paused and playing.
    pub fn toggle_pause(&mut self) {
        self.state = match &self.state {
            State::Playing => State::Paused,
            State::Paused => State::Playing,
            _ => self.state.to_owned(),
        }
    }
}
