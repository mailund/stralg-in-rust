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
        match size {
            0..<u8 as CharacterTrait>::MAX => Ok(CharSize::U8),
            <u8 as CharacterTrait>::MAX..<u16 as CharacterTrait>::MAX => Ok(CharSize::U16),
            _ => Err("Alphabet too large for known Char types".into()),
        }
    }
}
