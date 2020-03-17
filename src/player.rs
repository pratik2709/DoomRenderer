#[derive(Debug)]
pub struct Player {
    playerID: i16,
    xPosition: i16,
    yPosition: i16,
    angleOfPlayer: i16,
}

impl Player {
    fn new(playerID: i16) -> Player {
        Player {
            playerID,
            xPosition: 100,
            yPosition: 100,
            angleOfPlayer: 0,
        }
    }
}