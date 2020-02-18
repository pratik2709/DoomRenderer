include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");

extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Game", 320, 240).resizable().build().unwrap();
    let mut eventPump = sdl.event_pump().unwrap();
    'main: loop{
        for event in eventPump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
    let w = Wad::from_path("./DOOM1.wad");
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}