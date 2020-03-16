#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Copy + Ord,
{
    pub fn new(_input: &[T]) -> Self {
        let mut data = _input.to_vec();
        data.sort();
        data.dedup();

        Self { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.data.push(_element);
            self.data.sort();
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        match (self.is_empty(), _other.is_empty()) {
            (true, _) => true,
            (_, true) => false,
            _ => self.data.iter().all(|e| _other.contains(e)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        _other.data.iter().all(|e| !self.contains(e))
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        Self {
            data: _other
                .data
                .iter()
                .cloned()
                .filter(|e| self.contains(e))
                .collect(),
        }
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .filter(|e| !_other.contains(e))
            .collect();

        Self::new(&data[..])
    }

    pub fn union(&self, _other: &Self) -> Self {
        Self::new(&merge(&self, &_other)[..])
    }
}

fn merge<T>(rhs: &CustomSet<T>, lhs: &CustomSet<T>) -> Vec<T>
where
    T: Clone,
{
    rhs.data
        .iter()
        .cloned()
        .chain(lhs.data.iter().cloned())
        .collect()
}
