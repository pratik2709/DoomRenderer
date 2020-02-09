include!("wad.rs");


fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::Wad;

    #[test]
    fn open_file() {
        let w = Wad::from_path("./DOOM1.wad");
    }
}