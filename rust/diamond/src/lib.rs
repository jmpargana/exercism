pub fn get_diamond(c: char) -> Vec<String> {
    let len = (c as u8 - b'A') as i8;

    (-len..=len).map(|row| {
        (-len..=len).map(|col| {
            if row.abs() + col.abs() == len {
                (col.abs() as u8 + b'A') as char
            } else { ' ' }
        }).collect()
    }).collect()
}
