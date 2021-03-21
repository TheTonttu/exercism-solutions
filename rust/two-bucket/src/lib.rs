use std::collections::HashSet;

const MAX_PERMUTATION_ROUNDS: u8 = 42;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

impl Bucket {
    fn other(&self) -> Bucket {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }

    fn index(&self) -> usize {
        match self {
            Bucket::One => 0,
            Bucket::Two => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Empty(Bucket),
    Fill(Bucket),
    PourFrom(Bucket),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct BucketContainer {
    content: u8,
    capacity: u8,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    buckets: Vec<BucketContainer>,
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
    let mut moves: u8 = 0;

    let mut unique_permutations = HashSet::new();

    let opposite_start_state = create_start_state(&start_bucket.other(), capacity_1, capacity_2);
    // Opposite of start state is never allowed so mark it as a permutation we have already gone through so we never use it
    unique_permutations.insert(opposite_start_state);

    let mut history: Vec<Vec<State>> = Vec::new();

    while moves < MAX_PERMUTATION_ROUNDS {
        println!("{:?}", moves);

        // Brute force by going through every move permutation that we haven't already gone through until we hit the goal.
        match history.last() {
            Some(last_permutations) => {
                let mut curr_permutations = Vec::new();
                moves += 1;
                for state in last_permutations {
                    println!("prev state: {:?}", state);

                    for new_permutation in
                        generate_next_permutations(state, &mut unique_permutations)
                    {
                        if let Some((goal_bucket, goal_bucket_index)) =
                            goal_bucket(&new_permutation, goal)
                        {
                            let other_bucket_container = &new_permutation.buckets
                                [get_other_bucket_index(&goal_bucket_index)];

                            return Some(BucketStats {
                                moves,
                                goal_bucket,
                                other_bucket: other_bucket_container.content,
                            });
                        }

                        curr_permutations.push(new_permutation);
                    }
                }
                println!("{:?}", curr_permutations);
                history.clear();
                history.push(curr_permutations);
            }
            None => {
                moves += 1;

                let start_state = create_start_state(start_bucket, capacity_1, capacity_2);

                println!("start state: {:?}", start_state);

                if let Some((goal_bucket, goal_bucket_index)) = goal_bucket(&start_state, goal) {
                    let other_bucket_container =
                        &start_state.buckets[get_other_bucket_index(&goal_bucket_index)];

                    return Some(BucketStats {
                        moves,
                        goal_bucket,
                        other_bucket: other_bucket_container.content,
                    });
                }

                history.push(vec![start_state]);
            }
        }
    }

    None
}

fn create_start_state(start_bucket: &Bucket, capacity_1: u8, capacity_2: u8) -> State {
    // Start fill
    let start_contents = match start_bucket {
        Bucket::One => [capacity_1, 0],
        Bucket::Two => [0, capacity_2],
    };

    State::new(
        start_contents.iter().as_ref(),
        [capacity_1, capacity_2].iter().as_ref(),
    )
}

fn goal_bucket(state: &State, goal: u8) -> Option<(Bucket, usize)> {
    state
        .buckets
        .iter()
        .enumerate()
        .find(|(_i, c)| c.content == goal)
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

fn generate_next_permutations(
    state: &State,
    unique_permutations: &mut HashSet<State>,
) -> Vec<State> {
    let mut next_states = Vec::new();

    for (bucket_index, _bucket) in state.buckets.iter().enumerate() {
        for a_move in all_bucket_moves(&bucket_index) {
            let potential_state = state.after_move(&a_move);

            if state.buckets != potential_state.buckets
                // if we have not already encountered this state then go through it, otherwise skip it as the outcome would be same as before
                && unique_permutations.insert(potential_state.clone())
            {
                println!("next move: {:?}", a_move);
                println!("next state: {:?}", potential_state);
                next_states.push(potential_state)
            }
        }
    }

    next_states
}

fn all_bucket_moves(bucket_index: &usize) -> Vec<Move> {
    let bucket = match bucket_index {
        0 => Bucket::One,
        1 => Bucket::Two,
        _ => unimplemented!("unsupported bucket index {}", bucket_index),
    };

    vec![
        Move::Empty(bucket),
        Move::Fill(bucket),
        Move::PourFrom(bucket),
    ]
}

impl BucketContainer {
    fn new(content: u8, capacity: u8) -> Self {
        BucketContainer { content, capacity }
    }

    fn fill(&mut self) {
        self.content = self.capacity;
    }

    fn empty(&mut self) {
        self.content = 0;
    }

    fn pour_into(&mut self, other: &mut BucketContainer) {
        let target_space = other.capacity - other.content;
        let source_left = (self.content as i8 - target_space as i8).max(0) as u8;
        let target_add = self.content - source_left;

        self.content = source_left;
        other.content += target_add;
    }
}

impl State {
    fn new(bucket_contents: &[u8], bucket_capacities: &[u8]) -> Self {
        State {
            buckets: bucket_contents
                .iter()
                .zip(bucket_capacities.iter())
                .map(|(content, capacity)| BucketContainer::new(*content, *capacity))
                .collect(),
        }
    }

    fn from_buckets(buckets: &[BucketContainer]) -> Self {
        State {
            buckets: buckets.iter().copied().collect(),
        }
    }

    fn after_move(&self, selected_move: &Move) -> State {
        match selected_move {
            Move::Fill(bucket) => {
                let mut new_buckets = self.buckets.clone();
                new_buckets[bucket.index()].fill();

                State::from_buckets(&new_buckets)
            }
            Move::Empty(bucket) => {
                let mut new_buckets = self.buckets.clone();
                new_buckets[bucket.index()].empty();

                State::from_buckets(&new_buckets)
            }
            Move::PourFrom(bucket) => {
                let source_index = bucket.index();
                let target_index = bucket.other().index();

                let mut new_buckets = self.buckets.clone();

                // Cannot take 2 mutable references to vector so let's just clone for now.
                let mut source_bucket = new_buckets[source_index];
                let mut target_bucket = new_buckets[target_index];
                source_bucket.pour_into(&mut target_bucket);

                new_buckets[source_index] = source_bucket;
                new_buckets[target_index] = target_bucket;

                State::from_buckets(&new_buckets)
            }
        }
    }
}
