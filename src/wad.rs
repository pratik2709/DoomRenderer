use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;
use std::mem;

impl Wad {
    pub fn from_path(path: &str) -> Wad {
        // & str is called a string slice, an immutable view of a string
        let path = Path::new(path);
        let wadFile = File::open(path).unwrap_or_else(|e| {
            panic!("unable to open th WAD file {}", e)
        });
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

        let mapName = String::from("E1M1");
        w.loadMapData(&wadFile, mapName);
        w


    }

    pub fn loadMapData(&self, wadFile: &File, mapName: String) -> bool {
        let vertexMapData = self.readVertexMapData(wadFile, mapName);
        true
    }

    pub fn findMapIndex(&self, mapName: String) -> Option<usize> {
        for x in 0..self.directories.len() {
            if x == 10{
                break
            }
            match self.directories[x].lumpName == "E1M1" {
                true => {
                    println!("any luck");
                    return Some(x)
                },
                _ => continue
            };
        }
        None
    }

    pub fn readVertexMapData(&self, wadFile: &File, mapName: String) -> bool {
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

                        for x in 0..iVertexCount{
                            self.readVertexData(wadFile, self.header.directoryOffset + x * iVertexSizeInBytes);
                        }

                    }
                }
                false
            }
        };


        true
    }

    pub fn readVertexData(&self, mut file: &File, offset: usize){
        file.seek(SeekFrom::Start(offset as u64)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let mut raw_data:[u8;4] = [0;4];
        file.read_exact(&mut raw_data)
            .unwrap_or_else(|e|
            panic!("unable to read lump data {}", e));
//        println!("{:?}", raw_data);
        let xPosition = read2Bytes(&raw_data[0..2]) as i16;
        let yPosition = read2Bytes(&raw_data[2..4]) as i16;
        println!("{}, {}", xPosition, yPosition);

    }
}
