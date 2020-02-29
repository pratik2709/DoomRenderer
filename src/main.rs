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

extern crate sdl2;
fn main() {

    let mut game = Game::new();
    game.init();
    game.render();


//    let mut eventPump = sdl.event_pump().unwrap();
//    DoomEngine::keyPressed(& mut eventPump);

    let w = Wad::from_path(DoomEngine::getFileName());
}
