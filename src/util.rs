pub fn u8_to_u32(u8s: &[u8]) -> u32{
    if u8s.len() != 4{
        panic!("fail!");
    }
        //x | y Does a "bitwise or".
        // Each bit of the output is 0 if the corresponding bit of x
        // AND of y is 0, otherwise it's 1.

    let p1 = u8s[0];
    let p2 = u8s[1] << 8;
    let p2 = u8s[2] << 16;
    let p2 = u8s[3] << 24;

    p1 | p2 | p3 | p4

}