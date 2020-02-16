use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;

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
        let iMapIndex = self.findMapIndex(mapName).unwrap();

        println!("{}", iMapIndex);
//        match iMapIndex {
//            None => false,
//            mapName => {
//                false
////                iMapIndex += EMAPSLUMPSINDEX::eVERTEXES as usize;
////                match self.directories[iMapIndex]{
////
////                }
////                if self.directories[iMapIndex] != "VERTEXES" {
////                    return false;
////                }
//            }
//        };


        true
    }
}
