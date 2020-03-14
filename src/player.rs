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
            xPosition:0,
            yPosition:0,
            angleOfPlayer:0,
        }
    }
}