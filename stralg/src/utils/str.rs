use super::Alphabet;
use std::rc::Rc;

/// A string type that uses a custom alphabet for character encoding.
pub struct Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub alphabet: Rc<Alphabet>,
    chars: Vec<Char>,
}

impl<Char> Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    /// Creates a new `Str` from a given alphabet and a vector of characters.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    /// * `chars` - A vector of characters.
    ///
    /// # Returns
    ///
    /// A new `Str` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{Alphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(Alphabet::new(&['a', 'b', 'c']));
    /// let chars = vec![1u8, 2, 3];
    /// let s = Str::new(&alphabet, chars);
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn new(alphabet: &Rc<Alphabet>, chars: Vec<Char>) -> Self {
        Self {
            alphabet: alphabet.clone(),
            chars,
        }
    }

    /// Creates a new `Str` from a string slice.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to convert.
    ///
    /// # Returns
    ///
    /// A new `Str` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Str;
    ///
    /// let s = Str::<u8>::from_str("abc").unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = Rc::new(Alphabet::from_str(s));
        let chars = alphabet.translate::<Char>(s)?;
        Ok(Self::new(&alphabet, chars))
    }

    /// Creates a new `Str` from a string slice and a given alphabet.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to convert.
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    ///
    /// # Returns
    ///
    /// A new `Str` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{Alphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(Alphabet::new(&['a', 'b', 'c']));
    /// let s = Str::<u8>::from_str_with_alphabet("abc", &alphabet).unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str_with_alphabet(
        s: &str,
        alphabet: &Rc<Alphabet>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = alphabet.translate::<Char>(s)?;
        Ok(Self::new(alphabet, chars))
    }
}

impl<Char> std::ops::Index<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl<Char> std::ops::IndexMut<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chars[index]
    }
}
