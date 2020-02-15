#[derive(Debug)]
pub struct Directory {
    lumpOffset: usize,
    lumpSize: usize,
    lumpName: String,
}

#[derive(Debug)]
pub enum WadType {
    IWAD,
    PWAD,
    WAD2,
}


#[derive(Debug)]
pub struct Header {
    wadType: WadType,
    directoryCount: usize,
    directoryOffset: usize,
}

#[derive(Debug)]
pub struct Wad{
    header: Header,
    directory: Directory
}