//! Utils for string manipulation

use crate::authentication::Mechanism;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Encode a string as xtext
///
/// xtext is defined in <https://www.rfc-editor.org/rfc/rfc3461>
#[derive(Debug)]
pub struct XText<'a>(pub &'a str);

impl Display for XText<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for c in self.0.chars() {
            if c < '!' || c == '+' || c == '=' {
                write!(f, "+{:X}", c as u8)?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

#[doc = "возвращет вектор всех механизмов аутификации"]
pub fn get_all_mechanism() -> Vec<Mechanism> {
    let mut result: Vec<Mechanism> = Vec::new();
    result.push(Mechanism::Login);
    result.push(Mechanism::Plain);
    result.push(Mechanism::Xoauth2);
    result
}

#[cfg(test)]
mod tests {
    use super::XText;

    #[test]
    fn test() {
        for (input, expect) in [
            ("bjorn", "bjorn"),
            ("bjørn", "bjørn"),
            ("Ø+= ❤️‰", "Ø+2B+3D+20❤️‰"),
            ("+", "+2B"),
        ] {
            assert_eq!(format!("{}", XText(input)), expect.to_string());
        }
    }
}
