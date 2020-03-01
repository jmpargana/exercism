use std::cmp::Ordering;


pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let (mut start, mut end) = (0, array.len());

    while start <= end {
        let middle = (end + start) / 2;
        if array.len() <= middle { return None }

        match key.cmp(array.get(middle)?) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => start = middle + 1,
            Ordering::Less => {
                if middle <= 0 { return None }
                else { end = middle - 1; }
            }
        }
    }
    None
}
