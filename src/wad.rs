use std::fs::File;
use std::path::Path;
use std::io::*;
use std::str;

impl Wad{
    pub fn from_path(path: &str) -> Wad{
        println!("here");
        // & str his is called a string slice, an immutable view of a string
        let path = Path::new(path);
        let wadFile = File::open(path).unwrap_or_else(|e| {
           panic!("unable to open th WAD file {}", e)
        });
        let header = Header::from_file(&wadFile);
        println!("{:?}", header);

        // todo: implement a directory
        let directory = Directory::readDirectoryData(&wadFile, &header, header.directoryOffset);

        //loop through all directories
        for x in 0..header.directoryCount {
            println!("{:?}", Directory::readDirectoryData(
                &wadFile,
                &header,
                header.directoryOffset + x * 16));
            if x == 10 {
                break;
            }
        }

        Wad{
            header,
            directory
        }

    }
}
