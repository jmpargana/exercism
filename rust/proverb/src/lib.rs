use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    let mut result = String::new();
    for i in 1..list.len() {
        write!(result, "For want of a {} the {} was lost.\n", list[i-1], list[i]).unwrap();
    }
    write!(result, "And all for the want of a {}.", list[0]).unwrap();
    result

}
