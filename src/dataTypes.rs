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
pub struct Wad {
    header: Header,
    directories: Vec<Directory>,
}

#[derive(Debug)]
pub struct LineDef {
    startVertex: u16,
    endVertex: u16,
    flags: u16,
    lineType: u16,
    sectorTag: u16,
    rightSideDef: u16,
    leftSideDef: u16,

}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    xPosition: i16,
    yPosition: i16,
}

enum EMAPSLUMPSINDEX
{
    ENAME,
    ETHINGS,
    ELINEDEFS,
    ESIDEDDEFS,
    EVERTEXES,
    ESEAGS,
    ESSECTORS,
    ENODES,
    ESECTORS,
    EREJECT,
    EBLOCKMAP,
    ECOUNT,
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
    EDRAW = 128,
}

#[derive(Debug)]
pub struct Thing {
    xPosition: i16,
    yPosition: i16,
    angleOfThing: i16,
    typeOfThing: i16,
    flags: u16,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    xPartition: i16,
    yPartition: i16,
    changeXPartition: i16,
    changeYPartition: i16,

    rightBoxTop: i16,
    rightBoxBottom: i16,
    rightBoxLeft: i16,
    rightBoxRight: i16,

    leftBoxTop: i16,
    leftBoxBottom: i16,
    leftBoxLeft: i16,
    leftBoxRight: i16,

    rightChildID: u16,
    leftChildID: u16,
}


pub struct SubSector {
    segCount: u16,
    firstSegID: u16

}

pub struct Seg {
    startVertexID: u16
}

