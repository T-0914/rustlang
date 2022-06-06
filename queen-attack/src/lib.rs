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
    const MAX_RANK: i32 = 8;
    const MAX_FILE: i32 = 8;
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // unimplemented!(
        //     "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
        //     rank,
        //     file
        // );
        match (rank >= 0 && rank < ChessPosition::MAX_RANK)
            && (file >= 0 && file < ChessPosition::MAX_FILE)
        {
            true => Some(ChessPosition { rank, file }),
            false => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // unimplemented!(
        //     "Given the chess position {:?}, construct a Queen struct.",
        //     position
        // );
        Self {
            rank: position.rank,
            file: position.file,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // unimplemented!(
        //     "Determine if this Queen can attack the other Queen {:?}",
        //     other
        // );
        let sign_of_ranks: i32 = (self.rank - other.rank).abs();
        let sign_of_file: i32 = (self.file - other.file).abs();
        let is_on_the_same_row: bool = sign_of_ranks == 0;
        let is_on_the_same_column = sign_of_file == 0;
        let is_on_the_same_diagonal: bool = sign_of_ranks == sign_of_file;

        if is_on_the_same_row || is_on_the_same_column || is_on_the_same_diagonal {
            true
        } else {
            false
        }
    }
}
