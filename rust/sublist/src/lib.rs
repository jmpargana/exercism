#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let is_sub = |(a, b, x): (&[T], &[T], usize)| 
        b.is_empty() || a.windows(x).any(|w| w == b);

    match (first.len(), second.len()) {
        (x,y) if x == y && first == second => Comparison::Equal,
        (x,y) if x < y && is_sub((second, first, x)) => Comparison::Sublist,
        (x,y) if x > y && is_sub((first, second, y)) => Comparison::Superlist,
        (_,_) => Comparison::Unequal,
    }
}
