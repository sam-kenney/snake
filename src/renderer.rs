//! Render the game state to the screen using SDL2.
use crate::game::Game;
use crate::models::{Coordinate, State};
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

/// Render the game state to the screen using SDL2.
pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    /// Create a new renderer for the given window.
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    /// Draw a pixel at the given coordinate.
    fn draw_rect(&mut self, coord: &Coordinate) -> Result<(), String> {
        let Coordinate(x, y) = coord;
        self.canvas.fill_rect(Rect::new(
            x * crate::DOT_SIZE_IN_PXS as i32,
            y * crate::DOT_SIZE_IN_PXS as i32,
            crate::DOT_SIZE_IN_PXS,
            crate::DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    /// Draw the game state to the screen.
    pub fn draw(&mut self, game: &Game) -> Result<(), String> {
        self.draw_background(game);
        self.draw_player(game)?;
        self.draw_food(game)?;
        self.canvas.present();

        Ok(())
    }

    /// Draw the background of the game.
    fn draw_background(&mut self, game: &Game) {
        let color = match game.state {
            State::Playing => Color::RGB(0, 0, 0),
            State::Paused => Color::RGB(30, 30, 30),
            State::Over => Color::RGB(200, 50, 50),
            State::Win => Color::RGB(0, 200, 50),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Draw the snake to the screen.
    fn draw_player(&mut self, game: &Game) -> Result<(), String> {
        for (i, coord) in game.player_position.clone().into_iter().enumerate() {
            if i == 0 {
                // Draw snake's head
                self.canvas.set_draw_color(Color::RGB(204, 204, 255));
            } else if i % 2 == 0 {
                // Alternate colours for snake's body
                self.canvas.set_draw_color(Color::RGB(255, 51, 204));
            } else {
                self.canvas.set_draw_color(Color::RGB(255, 0, 255));
            }
            self.draw_rect(&coord.clone())?;
        }

        Ok(())
    }

    /// Draw the food to the screen.
    fn draw_food(&mut self, game: &Game) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(204, 0, 0));
        self.draw_rect(&game.food)?;
        Ok(())
    }
}
