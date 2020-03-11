pub struct Triangle([u64; 3]);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let result = sides.iter().all(|&side| side > 0)
            && sides
                .iter()
                .enumerate()
                .all(|(i, &side)| side <= sides.iter().cycle().skip(i + 1).take(2).sum());

        match result {
            false => None,
            _ => Some(Triangle(sides)),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.windows(2).all(|w| w[0] == w[1])
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.windows(2).any(|w| w[0] == w[1]) || self.0[0] == self.0[2]
    }
}
