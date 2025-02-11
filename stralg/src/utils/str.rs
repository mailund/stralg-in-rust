use super::{AlphabetChar, SizedAlphabet};
use std::rc::Rc;

/// A string type that uses a custom alphabet for character encoding.
pub struct Str<Char: AlphabetChar> {
    pub alphabet: Rc<SizedAlphabet<Char>>,
    chars: Vec<Char>,
}

impl<Char: AlphabetChar> Str<Char> {
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
    /// use stralg::utils::{SizedAlphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']));
    /// let chars = vec![1u8, 2, 3];
    /// let s = Str::new(&alphabet, chars);
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn new(alphabet: &Rc<SizedAlphabet<Char>>, chars: Vec<Char>) -> Self {
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
        let alphabet = Rc::new(SizedAlphabet::from_str(s));
        let chars = alphabet.translate(s)?;
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
    /// use stralg::utils::{SizedAlphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']));
    /// let s = Str::<u8>::from_str_with_alphabet("abc", &alphabet).unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str_with_alphabet(
        s: &str,
        alphabet: &Rc<SizedAlphabet<Char>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = alphabet.translate(s)?;
        Ok(Self::new(alphabet, chars))
    }

    /// Creates a new `Str` from a string slice using the same alphabet
    /// as the current `Str`. This maps the  two strings to the same string-space,
    /// so they can be manipulated together.
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
    /// use stralg::utils::{SizedAlphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']));
    /// let chars = vec![1u8, 2, 3];
    /// let s1 = Str::new(&alphabet, chars);
    /// let s2 = s1.to_shared_alphabet("abc").unwrap();
    /// assert_eq!(s2[0], 1);
    /// assert_eq!(s2[1], 2);
    /// assert_eq!(s2[2], 3);
    /// ```
    pub fn to_shared_alphabet(&self, s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = self.alphabet.clone();
        let chars = alphabet.translate(s)?;
        Ok(Self::new(&alphabet, chars))
    }
}

impl<Char: AlphabetChar> std::ops::Index<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl<Char: AlphabetChar> std::ops::IndexMut<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chars[index]
    }
}
