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

        for score in self.scores.iter().skip(3) {
            let lowest_index = get_lowest_value_index(&top3_scores);
            if *score > top3_scores[lowest_index] {
                top3_scores[lowest_index] = *score;
            }
        }

        top3_scores.sort_by(highest_to_lowest);

        top3_scores
    }
}

fn get_lowest_value_index(values: &[u32]) -> usize {
    let index = values
        .iter()
        .enumerate()
        .min_by(|&(_, value1), &(_, value2)| value1.cmp(value2))
        .unwrap_or((usize::MIN, &u32::MIN))
        .0;

    index
}

fn highest_to_lowest(a: &u32, b: &u32) -> Ordering {
    a.cmp(b).reverse()
}
