use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;

#[derive(Debug)]
pub enum WadType {
    IWAD,
    PWAD,
    WAD2,
}


//
//struct Directory
//{
//    uint32_t LumpOffset;
//    uint32_t LumpSize;
//    char LumpName[9];
//};

#[derive(Debug)]
pub struct Header {
    wadType: WadType,
    directoryCount: usize,
    directoryOffset: usize,
}

pub struct Directory {
    lumpOffset: u32,
    lumpSize: u32,
    lumpName: [char; 9],
}

impl Header {
    fn from_file(mut file: &File) -> Header {
        println!("loading WAD file..");
        // move the cursor 0 bytes from the start of the file
        file.seek(SeekFrom::Start(0)).unwrap_or_else(|e|
            panic!("unable to seek from the start od the file {}", e));

        let mut header_raw: [u8; 12] = [0;12];
        file.read_exact(&mut header_raw).unwrap_or_else(|e|
            panic!("unable to read the WAD header {}", e));

        let f = str::from_utf8(&header_raw[0..4]).unwrap();
        println!("This is the header {:?}", f);

        let wadType = match &header_raw[0..4] {
            b"IWAD" => WadType::IWAD,
            b"PWAD" => WadType::PWAD,
            b"WAD2" => WadType::WAD2,
            _ => panic!("could not read header.")
        };

        //usize is pointer-sized,
        // thus its actual size depends on the architecture
        // your are compiling your program for.
        let num_lumps: usize = u8_to_u32(&header_raw[4..8]) as usize;
        let directory_size: usize = u8_to_u32(&header_raw[8..12]) as usize;


        Header{
            wadType,
            directoryCount: num_lumps,
            directoryOffset: directory_size,
        }
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
        println!("{:?}", header)

    }
}

//fn wadLoader(path: String) {}
//
//fn loadWad() -> bool {}
//
//
//fn readDirectories() {}