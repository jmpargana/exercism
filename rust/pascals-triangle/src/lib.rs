pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).map(|row| {
            let mut start = vec![1];

            for k in 1..(row + 1) {
                match start.last() {
                    Some(&last) => start.push((last * (row + 1 - k)) / k),
                    None => ()
                }
            }
            start
        }).collect()
    }
}
