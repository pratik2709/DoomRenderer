include!("wad.rs");
include!("util.rs");
include!("directory.rs");
include!("header.rs");
include!("dataTypes.rs");
include!("map.rs");


fn main() {
    println!("Hello, world!");
    let w = Wad::from_path("./DOOM1.wad");

//    Map map("E1M1");
    let mapName = String::from("E1M1");
    w.loadMapData(mapName);
//    let map = Map{
//        name:String::from("E1M1"),
//    };
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");

    }
}