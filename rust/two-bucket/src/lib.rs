#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(Debug)]
enum Move {
    EmptyBucket(usize),
    FillBucket(usize),
    PourFrom(usize),
}

#[derive(Debug)]
struct State {
    bucket_contents: [u8; 2],
    bucket_capacities: [u8; 2],
}

impl State {
    fn new(bucket_contents: [u8; 2], bucket_capacities: [u8; 2]) -> Self {
        State {
            bucket_contents,
            bucket_capacities,
        }
    }

    fn after_move(&self, selected_move: &Move) -> State {
        match selected_move {
            Move::FillBucket(bucket_index) => {
                let mut new_contents = self.bucket_contents;
                new_contents[*bucket_index] = self.bucket_capacities[*bucket_index];
                State {
                    bucket_contents: new_contents,
                    bucket_capacities: self.bucket_capacities,
                }
            }
            Move::EmptyBucket(bucket_index) => {
                let mut new_contents = self.bucket_contents;
                new_contents[*bucket_index] = 0;
                State {
                    bucket_contents: new_contents,
                    bucket_capacities: self.bucket_capacities,
                }
            }
            Move::PourFrom(bucket_index) => {
                let source_index = match bucket_index {
                    0 => 0,
                    1 => 1,
                    _ => unimplemented!("unsupported index: {}", bucket_index),
                };

                let target_index = match bucket_index {
                    0 => 1,
                    1 => 0,
                    _ => unimplemented!("unsupported index: {}", bucket_index),
                };

                let target_space =
                    self.bucket_capacities[target_index] - self.bucket_contents[target_index];
                let source_left =
                    (self.bucket_contents[source_index] as i8 - target_space as i8).max(0) as u8;
                let source_remove = self.bucket_contents[source_index] - source_left;
                let target_content = self.bucket_contents[target_index] + source_remove;

                let mut new_contents = self.bucket_contents;
                new_contents[source_index] = source_left;
                new_contents[target_index] = target_content;

                State {
                    bucket_contents: new_contents,
                    bucket_capacities: self.bucket_capacities,
                }
            }
        }
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // Start fill
    let start_contents = match start_bucket {
        Bucket::One => [capacity_1, 0],
        Bucket::Two => [0, capacity_2],
    };

    let start_bucket_index = match start_bucket {
        Bucket::One => 0,
        Bucket::Two => 1,
    };

    // First fill counts as a move.
    let mut moves: u8 = 1;

    let start_state = State::new(start_contents, [capacity_1, capacity_2]);

    // TODO: No need to store the whole history, just one iteration.
    let mut history: Vec<Vec<State>> = Vec::new();
    history.push(vec![start_state]);

    while moves < u8::MAX {
        println!("{:?}", moves);

        // Brute force by going through every move permutation until we hit the goal.
        if let Some(last_permutations) = history.last() {
            let mut curr_permutations = Vec::new();
            moves += 1;
            for state in last_permutations {
                //println!("{:?}", state);
                for a_move in all_legal_moves(state, &start_bucket_index) {
                    let new_state = state.after_move(&a_move);

                    if let Some((goal_bucket, goal_bucket_index)) = goal_bucket(&new_state, goal) {
                        let other_bucket =
                            new_state.bucket_contents[get_other_bucket_index(&goal_bucket_index)];

                        return Some(BucketStats {
                            moves,
                            goal_bucket,
                            other_bucket,
                        });
                    }

                    curr_permutations.push(new_state);
                }
            }
            println!("{:?}", curr_permutations);
            history.push(curr_permutations);
        }
    }

    None
}

fn goal_bucket(state: &State, goal: u8) -> Option<(Bucket, usize)> {
    state
        .bucket_contents
        .iter()
        .copied()
        .enumerate()
        .find(|(_i, c)| *c == goal)
        .map(|(i, _c)| match i {
            0 => (Bucket::One, i),
            1 => (Bucket::Two, i),
            _ => unimplemented!("unsupported index: {}", i),
        })
}

fn get_other_bucket_index(the_bucket_index: &usize) -> usize {
    match the_bucket_index {
        0 => 1,
        1 => 0,
        _ => unimplemented!("unsupported index: {}", the_bucket_index),
    }
}

fn all_legal_moves(state: &State, start_bucket_index: &usize) -> Vec<Move> {
    let mut moves = Vec::new();

    for (bucket_index, _) in state.bucket_contents.iter().enumerate() {
        let other_index = get_other_bucket_index(&bucket_index);

        let is_start_bucket = bucket_index == *start_bucket_index;
        let is_larger_bucket =
            state.bucket_capacities[bucket_index] > state.bucket_capacities[other_index];
        let is_larger_start_bucket = is_start_bucket && is_larger_bucket;

        for a_move in all_bucket_moves(&bucket_index) {
            let potential_state = state.after_move(&a_move);

            let is_empty = potential_state.bucket_contents[bucket_index] == 0;
            let is_full = potential_state.bucket_contents[bucket_index]
                == potential_state.bucket_capacities[bucket_index];
            let other_is_empty = potential_state.bucket_contents[other_index] == 0;
            let other_is_full = potential_state.bucket_contents[other_index]
                == potential_state.bucket_capacities[other_index];

            if state.bucket_contents != potential_state.bucket_contents
                //  when starting with the larger bucket full, you are NOT allowed at any point to have the smaller bucket full and the larger bucket empty
                && !((is_larger_start_bucket && is_empty && other_is_full)
                    || (!is_larger_start_bucket && is_full && other_is_empty))
            {
                moves.push(a_move)
            }
        }
    }

    moves
}

fn all_bucket_moves(bucket_index: &usize) -> Vec<Move> {
    vec![
        Move::EmptyBucket(*bucket_index),
        Move::FillBucket(*bucket_index),
        Move::PourFrom(*bucket_index),
    ]
}
