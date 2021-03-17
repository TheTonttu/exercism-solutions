#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

enum Move {
    FillBucket(usize),
    EmptyBucket(usize),
    PourFrom(usize),
}

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
    // TODO: Should we count the first fill?
    let mut moves: u8 = 1;

    let state = State::new(start_contents, [capacity_1, capacity_2]);

    while moves < u8::MAX {
        // TODO: State change, etc.

        if goal_achieved(&state, goal) {
            let goal_bucket = Bucket::One;
            let other_bucket = 0;

            return Some(BucketStats {
                moves,
                goal_bucket,
                other_bucket,
            });
        }

        moves += 1;
    }

    None
}

fn goal_achieved(state: &State, goal: u8) -> bool {
    state.bucket_contents.iter().any(|c| *c == goal)
}
