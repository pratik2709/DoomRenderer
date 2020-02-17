include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");

extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let w = Wad::from_path("./DOOM1.wad");
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}