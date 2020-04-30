use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct Column {
    name: String,
    data: Vec<String>,
    width: usize,
}

impl Column {
    pub fn from_strings(name: &str, data: Vec<String>) -> Column {
        let mut column = Column {
            name: name.to_owned(),
            data,
            width: 0,
        };
        column.width = std::cmp::max(
            column.name.len(),
            column.data.iter().map(|s| s.len()).max().unwrap(),
        );
        column
    }

    pub fn from_decimals(name: &str, data: &[Decimal]) -> Column {
        Column::from_strings(name, data.iter().cloned().map(dec_to_str).collect())
    }

    /// Header.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Content of each row.
    pub fn data(&self) -> &[String] {
        &self.data
    }

    /// Column width. Equivalent to the character length of the longest data
    /// element.
    pub fn width(&self) -> usize {
        self.width
    }
}

/// Formats decimal to string with two decimal places and comma thousands
/// separators.
fn dec_to_str(dec: Decimal) -> String {
    let mut magnitude = (dec.abs() * Decimal::new(100, 0))
        .round()
        .to_u64()
        .unwrap_or_else(|| panic!("magnitude of '{}' is too large", dec));

    /// Appends to `chars` the ones digit of `magnitude`, then divides
    /// `magnitude` by 10.
    fn push_digit(chars: &mut Vec<u8>, magnitude: &mut u64) {
        let digit = (*magnitude % 10) as u8;
        chars.push(digit + b'0');
        *magnitude /= 10;
    }

    // Handle decimal places, the decimal point, and the ones digit.
    let mut chars = Vec::new();
    push_digit(&mut chars, &mut magnitude);
    push_digit(&mut chars, &mut magnitude);
    chars.push(b'.');
    push_digit(&mut chars, &mut magnitude);

    // Handle remaining digits.
    let mut i = 4;
    while magnitude != 0 {
        if (i - 2) % 4 == 0 {
            chars.push(b',');
            i += 1;
        }
        push_digit(&mut chars, &mut magnitude);
        i += 1;
    }

    if dec.is_sign_negative() {
        chars.push(b'-');
    }
    let chars = chars.into_iter().rev().collect();
    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        dec, expected,
        case(Decimal::new(10000, 0), "10,000.00"),
        case(Decimal::new(-123456789, 2), "-1,234,567.89"),
        case(Decimal::new(-123456789, 4), "-12,345.68"),
    )]
    fn dec_to_str(dec: Decimal, expected: &str) {
        assert_eq!(super::dec_to_str(dec), expected);
    }
}
