use std::fs::File;
use std::path::Path;
use std::io::*;

pub enum WadType {
    IWAD,
    PWAD,
    WAD2,
}

//struct Header
//{
//    char WADType[5];
//    uint32_t DirectoryCount;
//    uint32_t DirectoryOffset;
//};
//
//struct Directory
//{
//    uint32_t LumpOffset;
//    uint32_t LumpSize;
//    char LumpName[9];
//};

pub struct Header {
    wadType: WadType,
    directoryCount: u32,
    directoryOffset: u32,
}

pub struct Directory {
    lumpOffset: u32,
    lumpSize: u32,
    lumpName: [char; 9],
}

impl Header {
    fn from_file(mut file: &File) -> bool {
        println!("loading WAD file..");
        // move the cursor 0 bytes from the start of the file
        file.seek(SeekFrom::Start(0)).unwrap_or_else(|e|
            panic!("unable to seek from the start od the file {}", e));

        let mut header_raw: [u8; 12] = [0;12];
        file.read_exact(&mut header_raw).unwrap_or_else(|e|
            panic!("unable to read the WAD header {}", e));


        true
    }
}

pub struct Wad{
    header: Header,
    directory: Directory
}

impl Wad{
    pub fn from_path(path: &str){
        println!("here");
        // & str his is called a string slice, an immutable view of a string
        let path = Path::new(path);
        let wadFile = File::open(path).unwrap_or_else(|e| {
           panic!("unable to open th WAD file {}", e)
        });
        let header = Header::from_file(&wadFile);

    }
}

//fn wadLoader(path: String) {}
//
//fn loadWad() -> bool {}
//
//
//fn readDirectories() {}