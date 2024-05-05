#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition { rank: r1, file: f1 } = self.position;
        let ChessPosition { rank: r2, file: f2 } = other.position;
        let dr = (r1 - r2).abs();
        let df = (f1 - f2).abs();
        dr == df || r1 == r2 || f1 == f2
    }
}
