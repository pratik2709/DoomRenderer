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

pub struct Vertex{
    xPosition: i16,
    yPosition: i16,
}

enum EMAPSLUMPSINDEX{
    eTHINGS=1,
    eLINEDEFS,
    eSIDEDDEFS,
    eVERTEXES,
    eSEAGS,
    eSSECTORS,
    eNODES,
    eSECTORS,
    eREJECT,
    eBLOCKMAP,
    eCOUNT
}

enum ELINEDEFFLAGS
{
    eBLOCKING = 0,
    eBLOCKMONSTERS = 1,
    eTWOSIDED = 2,
    eDONTPEGTOP = 4,
    eDONTPEGBOTTOM = 8,
    eSECRET = 16,
    eSOUNDBLOCK = 32,
    eDONTDRAW = 64,
    eDRAW = 128
}

pub struct Map{
    name: String,
    vertexes: Vec<Vertex>,
    lineDefs: Vec<LineDef>
}