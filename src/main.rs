use glfw::{Action, Context, Key};
use windowandkey::WindowAndKeyContext;
use std::time::{Instant};
use game::Game;

mod vec;
mod shader;
mod worldgeometry;
mod chunk;
mod cube;
mod packedvertex;
mod windowandkey;
mod game;
mod camera;
mod texture;
mod blockinfo;
mod fader;
mod collisioncage;

#[cfg(test)]
mod tests;

fn main() {
    let mut wak_context = WindowAndKeyContext::new("Barkaroo");
    
    let game = Game::new();

    wak_context.game = Some(game);
    wak_context.game.as_mut().unwrap().set_mouse_focused(true);
    wak_context.game.as_mut().unwrap().start_world();
    while !wak_context.window.should_close() {
        wak_context.run();
    }
}
