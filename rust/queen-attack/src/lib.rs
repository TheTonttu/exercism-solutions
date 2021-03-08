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
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (f1, r1) = (self.position.file, self.position.rank);
        let (f2, r2) = (other.position.file, other.position.rank);

        f1 == f2 || r1 == r2
            // Positions are diagonal if difference in coordinates is equal
            // |x1 - x2| == |y1 - y2|
            || (f1 - f2).abs() == (r1 - r2).abs()
    }
}
