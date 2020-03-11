#[derive(Debug, Copy, Clone)]
pub struct ChessPosition (i32, i32);

#[derive(Debug, Copy, Clone)]
pub struct Queen (ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (x, y) if x < 0 || y < 0 || x > 7 || y > 7 => None,
            _ => Some(Self(rank, file))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self (position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (self.0, other.0) {
            (x, y) if x.0 == y.0 || x.1 == y.1 || (x.0 - y.0).abs() == (x.1 - y.1).abs() 
                => true,
            _  => false
        }
    }
}
