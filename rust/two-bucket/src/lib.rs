#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(Debug)]
enum Move {
    FillBucket(usize),
    EmptyBucket(usize),
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
            for state in last_permutations {
                for a_move in all_legal_moves() {
                    println!("Before: {:?}", state);
                    println!("Move: {:?}", a_move);
                    let new_state = state.after_move(&a_move);
                    println!("After: {:?}", state);

                    if let Some((goal_bucket, goal_bucket_index)) = goal_bucket(&new_state, goal) {
                        let other_bucket =
                            new_state.bucket_contents[get_other_bucket_index(&goal_bucket_index)];
                        moves += 1;

                        return Some(BucketStats {
                            moves,
                            goal_bucket,
                            other_bucket,
                        });
                    }

                    curr_permutations.push(new_state);
                }
            }
            history.push(curr_permutations);
        }

        moves += 1;
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

fn all_legal_moves() -> Vec<Move> {
    // TODO: Pass state and check relevant moves.
    vec![
        Move::FillBucket(0),
        Move::FillBucket(1),
        Move::EmptyBucket(0),
        Move::EmptyBucket(1),
        Move::PourFrom(0),
        Move::PourFrom(1),
    ]
}
