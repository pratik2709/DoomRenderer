use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;
use std::mem;

impl Wad {
    pub fn from_path(path: &str) -> Wad {
        println!("here");
        // & str is called a string slice, an immutable view of a string
        let path = Path::new(path);
        let wadFile = File::open(path).unwrap_or_else(|e| {
            panic!("unable to open th WAD file {}", e)
        });
        let header = Header::from_file(&wadFile);
        println!("{:?}", header);

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

        Wad {
            header,
            directories: dirs,
        }
    }

    pub fn loadMapData(&self, mapName: String) -> bool {
        let vertexMapData = self.readVertexMapData(mapName);
        true
    }

    pub fn findMapIndex(&self, mapName: String) -> Option<usize> {
        for x in 0..self.directories.len() {
            println!("checking");
            match &self.directories[x] {
                mapName => Some(x),
                _ => continue
            };
        }
        None
    }

    pub fn readVertexMapData(&self, mapName: String) -> bool {
        let iMapIndex = self.findMapIndex(mapName);

        match iMapIndex {
            None => false,
            mapName => {
                let mut newIMapIndex = iMapIndex.unwrap();
                let vertexString = String::from("VERTEXES");
                newIMapIndex += EMAPSLUMPSINDEX::eVERTEXES as usize;
                match &self.directories[newIMapIndex].lumpName{
                    vertexString => {
                        let iVertexSizeInBytes = mem::size_of::<Vertex>();

                        //not understanding this
                        let iVertexCount =
                            self.directories[newIMapIndex].lumpSize/iVertexSizeInBytes;

//                        for x in 0..iVertexCount{
//
//                        }

                    }
                }
                false
            }
        };


        true
    }

    pub fn readVertexData(mut file: &File, offset: usize){
        file.seek(SeekFrom::Start(offset as u64)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let mut raw_data:[u8;8] = [0;8];
        file.read_exact(&mut raw_data)
            .unwrap_or_else(|e|
            panic!("unable to read lump data {}", e));
        let xPosition = u8_to_u32(&raw_data[0..4]) as usize;
        let yPosition = u8_to_u32(&raw_data[4..8]) as usize;

    }
}
