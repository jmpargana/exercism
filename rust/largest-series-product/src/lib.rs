#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 { return Ok(1) }

    Ok(string_digits.chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?
        .windows(span)
        .map(|span| span.iter().product::<u32>() as u64)
        .max()
        .ok_or(Error::SpanTooLong)?)
}
