pub struct Directory {
    lumpOffset: u32,
    lumpSize: u32,
    lumpName: [char; 9],
}

impl Directory{

    fn readDirectoryData(mut file: &File, header: &Header){
        file.seek(SeekFrom::Start(header::directoryOffset)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let raw_data = [u8;12];

    }

}