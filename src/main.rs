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
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Game", 320, 240).resizable().build().unwrap();

    //Canvas:
    // Manages and owns a target (Surface or Window) and allows drawing in it.
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    DoomEngine::render(& mut eventPump);


    let mut eventPump = sdl.event_pump().unwrap();
    DoomEngine::keyPressed(& mut eventPump);

    let w = Wad::from_path("./DOOM1.wad");
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}