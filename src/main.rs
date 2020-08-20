fn main() {
    let board = Board{};
    let mut history = Vec::new();
    history.push(board);
    let solutions = solve(history);
}

#[derive(Clone)]
struct Board {
}

type History = Vec<Board>;

fn solve(history: History) -> Vec<History> {
    let board = history.last().unwrap();
    let nextBoards = board.next(history.as_slice());
    let result = vec![];
    nextBoards.iter().for_each(|board| {
        let vec = vec![];
        vec.extend_from_slice(history.as_slice());
        vec.push(*board);
        result.append(&mut solve(vec));
    });

    return result;
}

impl Board {
    fn next(&self, history: &[Board]) -> Vec<Board> {
        /*
        let tiles = board.tiles
            .filter(hasBlocks)
            .filter(canAccess)
            .filter(playNotHere)
    
        return tiles
            .availableMoves
            .map(board(move: tile, direction: direction))
            .filter(notIn: history)        
        */
        return vec![];
    }
}
