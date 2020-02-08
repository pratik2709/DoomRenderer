use std::fs::File;

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
    fn openAndLoad(mut file: &File) -> bool {
        println!("loading WAD file..");
        true
    }
}

//fn wadLoader(path: String) {}
//
//fn loadWad() -> bool {}
//
//
//fn readDirectories() {}