#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T> 
    where T: PartialEq + Clone + Ord {
    pub fn new(_input: &[T]) -> Self {
        let mut data = _input.to_vec();
        data.sort();
        data.dedup();

        CustomSet { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains(&_element)
    }

    pub fn add(&mut self, _element: T) {
        match self.data.contains(&_element) {
            false => self.data.push(_element),
            _ => (),
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.data.iter().any(|e| _other.data.iter().all(|x| x != e))
            || self.data == _other.data
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.data.iter().all(|e| _other.data.iter().all(|x| x != e))
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        let mut data = self.data
                .iter()
                .filter(|e| !_other.data.contains(&e))
                .cloned()
                .collect::<Vec<T>>();

        data.sort();
        CustomSet { data }
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let mut data = self.data
                .iter()
                .filter(|e| _other.data.contains(&e))
                .cloned()
                .collect::<Vec<T>>();

        data.sort();
        CustomSet { data }
    }

    pub fn union(&self, _other: &Self) -> Self {
        CustomSet { [&self.data[..], &_other.data[..]].concat() } 
    }
}
