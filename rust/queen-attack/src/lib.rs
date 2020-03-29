#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            (_, _) => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            rank: position.rank,
            file: position.file,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let delta_rank = self.rank - other.rank;
        let delta_file = self.file - other.file;
        match (delta_rank, delta_file) {
            (0, _) => true,
            (_, 0) => true,
            (dx, dy) if dx == dy => true,
            (dx, dy) if dx == -dy => true,
            (_, _) => false,
        }
    }
}
