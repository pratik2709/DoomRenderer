//read 4 bytes
pub fn u8_to_u32(u8s: &[u8]) -> u32 {
    if u8s.len() != 4 {
        panic!("fail!");
    }
    //x | y Does a "bitwise or".
    // Each bit of the output is 0 if the corresponding bit of x
    // AND of y is 0, otherwise it's 1.

    //type conversion
    let p1 = u32::from(u8s[0]);
    let p2 = u32::from(u8s[1]) << 8;
    let p3 = u32::from(u8s[2]) << 16;
    let p4 = u32::from(u8s[3]) << 24;

    p1 | p2 | p3 | p4
}

//read 2 bytes
pub fn read2Bytes(u8s: &[u8]) -> u16 {
    let p1 = u16::from(u8s[0]);
    let p2 = u16::from(u8s[1]) << 8;
    p1 | p2
}

//uint16_t WADReader::Read2Bytes(const uint8_t *pWADData, int offset)
//{
//    return (pWADData[offset + 1] << 8) | pWADData[offset];
//}