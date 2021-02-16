use std::cmp::Ordering;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top3_scores: Vec<u32> = self.scores.iter().take(3).copied().collect();

        for i in 3..self.scores.len() {
            let score = self.scores[i];
            let lowest_index = get_lowest_value_index(&top3_scores);
            if score > top3_scores[lowest_index] {
                top3_scores[lowest_index] = score;
            }
        }

        top3_scores.sort_by(highest_to_lowest);

        top3_scores
    }
}

fn get_lowest_value_index(values: &[u32]) -> usize {
    let mut lowest = u32::MAX;
    let mut index = 0;
    for i in 0..values.len() {
        let curr_value = values[i];
        if curr_value < lowest {
            lowest = curr_value;
            index = i;
        }
    }
    index
}

fn highest_to_lowest(a: &u32, b: &u32) -> Ordering {
    a.cmp(b).reverse()
}
