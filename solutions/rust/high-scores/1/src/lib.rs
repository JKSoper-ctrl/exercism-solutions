#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_highscore_vec = self.0.clone();
        sorted_highscore_vec.sort_by(|a, b| b.cmp(a));
        let top_three_scores: Vec<u32> = sorted_highscore_vec.into_iter().take(3).collect();

        top_three_scores
    }
}
