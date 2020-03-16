#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<u16>,
    roll: bool,
}


impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: vec![], roll: false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let enough = pins > 10 
            || (self.roll && pins + self.frames.last().unwrap() > 10);

        match (enough, self.score().is_some()) {
            (true, _) => Err(Error::NotEnoughPinsLeft),
            (_, true) => Err(Error::GameComplete),
            (_, _) => {
                self.frames.push(pins);
                self.roll = match pins {
                    10 => false,
                    _ => !self.roll,
                };
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        let (mut score, mut i) = (0, 0);

        for _ in 0..10 {
            match (&self.frames.get(i), &self.frames.get(i + 1)) {
                (Some(&a), Some(&b)) => {
                    score += a + b;
                    if a == 10 || a + b == 10 {
                        match &self.frames.get(i + 2) {
                            Some(&c) => score += c,
                            None => return None,
                        }
                        i += if a == 10 { 1 } else { 2 };
                    }
                },
                _ => return None
            }
        }
        Some(score)
    }
}
