#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores_vec = scores.to_vec();
        HighScores { scores: scores_vec }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
