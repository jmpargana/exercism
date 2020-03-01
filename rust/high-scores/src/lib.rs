#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores 
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&x| x) 
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.clone().into_iter().max().map(|x| x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        scores.into_iter().rev().take(3).collect()
    }
}
