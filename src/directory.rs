#[derive(Debug)]
pub struct Directory {
    lumpOffset: usize,
    lumpSize: usize,
    lumpName: String,
}

impl Directory{

    fn readDirectoryData(mut file: &File, header: &Header) -> Directory{
        file.seek(SeekFrom::Start(header.getHeader() as u64)).unwrap_or_else(|e|
            panic!("unable to read directory data {}", e));

        let mut raw_data:[u8;16] = [0;16];
        //Read the exact number of bytes required to fill buf.
        file.read_exact(&mut raw_data)
            .unwrap_or_else(|e|
            panic!("unable to read lump data {}", e));

        let lumpOffset = u8_to_u32(&raw_data[0..4]) as usize;
        let lumpSize = u8_to_u32(&raw_data[4..8]) as usize;

        //convert to utf8 (ascii string)

        let lumpName:String = String::from_utf8(raw_data[8..16].to_vec())
            .unwrap_or_else(|e| panic!("unable to ascii string {}",e));

        println!("{:?}, {:?}, {:?}", lumpOffset, lumpName, lumpSize);
        Directory{
            lumpOffset,
            lumpSize,
            lumpName
        }

    }

}