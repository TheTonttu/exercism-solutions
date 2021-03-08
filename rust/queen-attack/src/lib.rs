#[derive(Debug)]
pub struct ChessPosition {
    file: i32,
    rank: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    const MAX_FILE: i32 = 7;
    const MAX_RANK: i32 = 7;

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match ChessPosition::is_valid_position(&rank, &file) {
            true => Some(Self { file, rank }),
            false => None,
        }
    }

    fn is_valid_position(rank: &i32, file: &i32) -> bool {
        (0..=ChessPosition::MAX_FILE).contains(file) && (0..=ChessPosition::MAX_RANK).contains(rank)
    }

    fn is_same_file(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    fn is_same_rank(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    fn is_diagonal_to(&self, other: &ChessPosition) -> bool {
        // Positions are diagonal if difference in coordinates is equal
        // |x1 - x2| == |y1 - y2|
        (self.file - other.file).abs() == (self.rank - other.rank).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.is_same_file(&other.position)
            || self.position.is_same_rank(&other.position)
            || self.position.is_diagonal_to(&other.position)
    }
}
