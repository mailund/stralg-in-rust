/// A trait for character types.
///
/// This trait is used to define the character types that can be used in the library.
pub trait CharacterTrait:
    Eq
    + std::hash::Hash
    + TryFrom<usize, Error: std::fmt::Debug>
    + Copy
    + Into<usize>
    + std::fmt::Debug
    + 'static
{
    const MAX: usize;
    fn to_usize(&self) -> usize {
        (*self).into()
    }
}
impl CharacterTrait for u8 {
    const MAX: usize = u8::MAX as usize - 1; // -1 to leave room for the sentinel
}
impl CharacterTrait for u16 {
    const MAX: usize = u16::MAX as usize - 1; // -1 to leave room for the sentinel
}

/// The size of characters needed to represent a given string.
#[derive(Debug, PartialEq)]
pub enum CharSize {
    /// 8 bits needed for each character
    U8,
    /// 16 bits needed for each character
    U16,
}

impl CharSize {
    /// Returns the character size needed to represent an alphabet of a given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the alphabet.
    ///
    /// # Returns
    ///
    /// The character size needed to represent the alphabet.
    ///
    /// # Errors
    ///
    /// Returns an error if the alphabet is too large for known character types.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{Alphabet, CharSize};
    ///
    /// let alphabet = Alphabet::from_str("abc");
    /// let result = alphabet.char_size().unwrap();
    /// assert_eq!(result, CharSize::U8);
    /// ```
    pub fn from_alphabet_size(size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        const U8_MAX: usize = <u8 as CharacterTrait>::MAX;
        const U16_MIN: usize = U8_MAX + 1;
        const U16_MAX: usize = <u16 as CharacterTrait>::MAX;
        match size {
            0..=U8_MAX => Ok(CharSize::U8),
            U16_MIN..U16_MAX => Ok(CharSize::U16),
            _ => Err("Alphabet too large for known Char types".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Alphabet;

    #[test]
    fn test_char_size_small() {
        let result = CharSize::from_alphabet_size(3).unwrap();
        assert_eq!(result, CharSize::U8);
    }

    #[test]
    fn test_char_size_still_small() {
        let result = CharSize::from_alphabet_size((u8::MAX - 1) as usize).unwrap();
        assert_eq!(result, CharSize::U8);
    }

    #[test]
    fn test_just_too_large_alphabet() {
        let result = CharSize::from_alphabet_size((u16::MAX) as usize);
        assert!(result.is_err());
    }

    #[test]
    fn test_way_too_large_alphabet() {
        let result = CharSize::from_alphabet_size((u32::MAX) as usize);
        assert!(result.is_err());
    }

    #[test]
    fn test_char_size_with_alphabet() {
        let alphabet = Alphabet::from_str("abc");
        let result = alphabet.char_size().unwrap();
        assert_eq!(result, CharSize::U8);
    }

    #[test]
    fn test_char_just_in_u8_with_alphabet() {
        let letters: Vec<char> = (0..(u8::MAX as u32 - 1))
            .map(|c| std::char::from_u32(c).unwrap())
            .collect();
        let alphabet = Alphabet::new(&letters);
        let result = alphabet.char_size().unwrap();
        assert_eq!(result, CharSize::U8);
    }

    #[test]
    fn test_char_size_just_barely_large_with_alphabet() {
        let letters: Vec<char> = (0..u8::MAX as u32)
            .map(|c| std::char::from_u32(c).unwrap())
            .collect();
        let alphabet = Alphabet::new(&letters);
        let result = alphabet.char_size().unwrap();
        assert_eq!(result, CharSize::U16);
    }
}
