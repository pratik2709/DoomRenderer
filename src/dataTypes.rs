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
    directories: Vec<Directory>
}

#[derive(Debug)]
pub struct LineDef{
    startVertex: u16,
    endVertex: u16,
    flags: u16,
    lineType: u16,
    sectorTag: u16,
    rightSideDef: u16,
    leftSideDef: u16,

}

#[derive(Debug)]
pub struct Vertex{
    xPosition: i16,
    yPosition: i16,
}

enum EMAPSLUMPSINDEX{
    ETHINGS=1,
    ELINEDEFS,
    ESIDEDDEFS,
    EVERTEXES,
    ESEAGS,
    ESSECTORS,
    ENODES,
    ESECTORS,
    EREJECT,
    EBLOCKMAP,
    ECOUNT
}

enum ELINEDEFFLAGS
{
    EBLOCKING = 0,
    EBLOCKMONSTERS = 1,
    ETWOSIDED = 2,
    EDONTPEGTOP = 4,
    EDONTPEGBOTTOM = 8,
    ESECRET = 16,
    ESOUNDBLOCK = 32,
    EDONTDRAW = 64,
    EDRAW = 128
}

