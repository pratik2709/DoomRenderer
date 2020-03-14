#[derive(Debug)]
pub struct Player{
    playerID: i32,
    xPosition: i16,
    yPosition: i16,
    angleOfPlayer: i32
}

impl Player{
    fn new(playerID: i32) -> Player{
        Player{
            playerID,
            xPosition:10,
            yPosition:10,
            angleOfPlayer:0,
        }
    }
}