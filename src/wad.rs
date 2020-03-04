use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;
use std::mem;

impl Wad {
    pub fn getWadData(wadFile: &File) -> Wad {
        let header = Header::from_file(&wadFile);

        // todo: implement a directory
        let directory = Directory::readDirectoryData(&wadFile, &header, header.directoryOffset);


        //loop through all directories
        let mut dirs: Vec<Directory> = Vec::new();
        for x in 0..header.directoryCount {
            dirs.push(Directory::readDirectoryData(
                &wadFile,
                &header,
                header.directoryOffset + x * 16));
        }

        let w = Wad {
            header,
            directories: dirs,
        };

        w
    }

    pub fn loadFileUsingPath(path: &str) -> File {
        // & str is called a string slice, an immutable view of a string
        let path = Path::new(path);
        let wadFile = File::open(path).unwrap_or_else(|e| {
            panic!("unable to open th WAD file {}", e)
        });
        wadFile
    }

    pub fn loadMapData(&self, wadFile: &File, mapName: String) -> Map {
        let s= mapName.clone();
        let s1= mapName.clone();
        let mut lineDefCollection: Vec<LineDef> = Vec::new();
        let mut vertexCollection: Vec<Vertex> = Vec::new();
        let vertexMapData = self.readVertexMapData(wadFile, mapName, &mut vertexCollection);
        let lineDefData = self.readMapLineDef(wadFile, s, &mut lineDefCollection);
        Map::new(s1, vertexCollection, lineDefCollection)
    }

    pub fn findMapIndex(&self, mapName: String) -> Option<usize> {
        for x in 0..self.directories.len() {
            if x == 10 {
                break
            }
            match self.directories[x].lumpName == "E1M1" {
                true => {
                    println!("any luck");
                    return Some(x);
                }
                _ => continue
            };
        }
        None
    }

    pub fn readVertexMapData(&self, wadFile: &File, mapName: String, vertexCollection:
    &mut Vec<Vertex>) -> bool{
        let iMapIndex = self.findMapIndex(mapName);

        match iMapIndex {
            None => false,
            mapName => {
                let mut newIMapIndex = iMapIndex.unwrap();
                let vertexString = String::from("VERTEXES");
                newIMapIndex += EMAPSLUMPSINDEX::EVERTEXES as usize;
                match &self.directories[newIMapIndex].lumpName {
                    vertexString => {
                        let iVertexSizeInBytes = mem::size_of::<Vertex>();

                        //not understanding this
                        let iVertexCount =
                            self.directories[newIMapIndex].lumpSize / iVertexSizeInBytes;

                        for x in 0..iVertexCount {
                            vertexCollection.push(self.readVertexData(wadFile, self
                                .directories[newIMapIndex]
                                .lumpOffset + x * iVertexSizeInBytes));
                        }
                    }
                }
                false
            }
        };


        true
    }

    pub fn readVertexData(&self, mut file: &File, offset: usize) -> Vertex {
        file.seek(SeekFrom::Start(offset as u64)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let mut raw_data: [u8; 4] = [0; 4];
        file.read_exact(&mut raw_data)
            .unwrap_or_else(|e|
                panic!("unable to read lump data {}", e));
//        println!("{:?}, {:?}", &raw_data[0..2] as &[u8], &raw_data);
        let xPosition = read2Bytes(&raw_data[0..2]) as i16;
        let yPosition = read2Bytes(&raw_data[2..4]) as i16;

        Vertex{
            xPosition, yPosition
        }
    }


    pub fn readMapLineDef(&self, wadFile: &File, mapName: String, lineDefCollection
    :&mut Vec<LineDef>) -> bool {
        let iMapIndex = self.findMapIndex(mapName);

        match iMapIndex {
            None => false,
            mapName => {
                let mut newIMapIndex = iMapIndex.unwrap();
                let vertexString = String::from("LINEDEFS");
                newIMapIndex += EMAPSLUMPSINDEX::ELINEDEFS as usize;
                match &self.directories[newIMapIndex].lumpName {
                    vertexString => {
                        let iVertexSizeInBytes = mem::size_of::<LineDef>();

                        //not understanding this
                        let iVertexCount =
                            self.directories[newIMapIndex].lumpSize / iVertexSizeInBytes;

                        for x in 0..iVertexCount {
                            lineDefCollection.push(self.ReadLinedefData(wadFile, self
                                .directories[newIMapIndex]
                                .lumpOffset + x * iVertexSizeInBytes));
                        }
                    }
                }
                false
            }
        };


        true
    }

    pub fn ReadLinedefData(&self, mut file: &File, offset: usize) -> LineDef {
        file.seek(SeekFrom::Start(offset as u64)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let mut raw_data: [u8; 14] = [0; 14];
        file.read_exact(&mut raw_data)
            .unwrap_or_else(|e|
                panic!("unable to read lump data {}", e));
//        println!("{:?}, {:?}", &raw_data[0..2] as &[u8], &raw_data);
        let startVertex = read2Bytes(&raw_data[0..2]) as u16;
        let endVertex = read2Bytes(&raw_data[2..4]) as u16;
        let flags = read2Bytes(&raw_data[4..6]) as u16;
        let lineType = read2Bytes(&raw_data[6..8]) as u16;
        let sectorTag = read2Bytes(&raw_data[8..10]) as u16;
        let rightSideDef = read2Bytes(&raw_data[10..12]) as u16;
        let leftSideDef = read2Bytes(&raw_data[12..14]) as u16;
        let l = LineDef {
            startVertex,
            endVertex,
            flags,
            lineType,
            sectorTag,
            rightSideDef,
            leftSideDef,

        };
        l
    }
}
