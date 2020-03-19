#![allow(non_snake_case)]

include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");
include!("game.rs");
include!("DoomEngine.rs");
include!("Player.rs");

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ops::Not;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::rc::Rc;
use std::cell::RefCell;

extern crate sdl2;

const SUBSECTORIDENTIFIER: i16 = 10;

fn main() {
    let mut game = Game::new();
    game.init();
    game.processInput();
}
