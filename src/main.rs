include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");


fn main() {
    println!("Hello, world!");
    let w = Wad::from_path("./DOOM1.wad");
//    let map = Map{
//
//    }
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}