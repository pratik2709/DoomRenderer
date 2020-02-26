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

    Game::new();
    Game::init();
    DoomEngine::render(canvas);


    let mut eventPump = sdl.event_pump().unwrap();
    DoomEngine::keyPressed(& mut eventPump);

    let w = Wad::from_path(DoomEngine::getFileName());
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}