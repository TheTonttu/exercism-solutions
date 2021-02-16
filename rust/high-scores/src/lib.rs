use std::cmp::Ordering;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32]
}

impl <'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores {
            scores
        }
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
        // Not very efficient...
        let mut top3_scores = self.scores.to_vec();
        top3_scores.sort_by(highest_to_lowest);
        top3_scores.truncate(3);

        top3_scores
    }
}

fn highest_to_lowest(a: &u32, b: &u32) -> Ordering {
    a.cmp(b).reverse()
}
