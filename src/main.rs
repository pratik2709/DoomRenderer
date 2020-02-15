include!("wad.rs");
include!("util.rs");
include!("directory.rs");


fn main() {
    println!("Hello, world!");
    let w = Wad::from_path("./DOOM1.wad");
//    println!("{:?}", w);
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");
    }
}