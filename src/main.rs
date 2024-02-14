//! A simple snake game written in Rust using SDL2.
mod game;
mod models;
mod renderer;
use game::Game;
use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

const GRID_X_SIZE: u32 = 20;
const GRID_Y_SIZE: u32 = 20;
const DOT_SIZE_IN_PXS: u32 = 20;
const SNAKE_SIZE_FOR_WIN: usize = ((GRID_Y_SIZE * GRID_X_SIZE) - 1) as usize;

/// Run the game loop.
fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window(
            "Snake",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut game = Game::new();
    let mut renderer = Renderer::new(window)?;
    let mut events = sdl.event_pump()?;
    let mut frame_counter = 0;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => game.move_up(),
                    Keycode::A | Keycode::Left => game.move_left(),
                    Keycode::S | Keycode::Down => game.move_down(),
                    Keycode::D | Keycode::Right => game.move_right(),
                    Keycode::R => game.reset(),
                    Keycode::Escape | Keycode::Space => game.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        // Run at ~30 FPS
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        // Update game state every 5 frames (otherwise snake way too fast)
        frame_counter += 1;
        if frame_counter % 5 == 0 {
            game.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&game)?;
    }

    Ok(())
}
