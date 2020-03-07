#![allow(non_snake_case)]

include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");
include!("game.rs");
include!("DoomEngine.rs");

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ops::Not;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

extern crate sdl2;

fn main() {
    let mut game = Game::new();
    game.init();
    game.processInput();
}
