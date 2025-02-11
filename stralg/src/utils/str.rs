use super::{Alphabet, CharacterTrait, SizedAlphabet};
use std::rc::Rc;

/// A string type that uses a custom alphabet for character encoding.
pub struct SizedStr<Char: CharacterTrait> {
    pub alphabet: Rc<SizedAlphabet<Char>>,
    char_vector: Vec<Char>,
}

impl<Char: CharacterTrait> SizedStr<Char> {
    /// Creates a new `Str` from a given alphabet and a vector of characters.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    /// * `x` - A vector of characters.
    ///
    /// # Returns
    ///
    /// A new `SizedStr` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{SizedAlphabet, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']).unwrap());
    /// let chars = vec![1u8, 2, 3];
    /// let s = SizedStr::new(&alphabet, chars);
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn new(alphabet: &Rc<SizedAlphabet<Char>>, x: Vec<Char>) -> Self {
        Self {
            alphabet: alphabet.clone(),
            char_vector: x,
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
    /// A new `SizedStr` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::SizedStr;
    ///
    /// let s = SizedStr::<u8>::from_str("abc").unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet: Rc<SizedAlphabet<Char>> = Rc::new(SizedAlphabet::from_str(s)?);
        Self::from_str_with_alphabet(s, &alphabet)
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
    /// A new `SizedStr` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{SizedAlphabet, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']).unwrap());
    /// let s = SizedStr::<u8>::from_str_with_alphabet("abc", &alphabet).unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str_with_alphabet(
        s: &str,
        alphabet: &Rc<SizedAlphabet<Char>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let x = s
            .chars()
            .map(|c| {
                alphabet
                    .index(c)
                    .ok_or_else(|| "Character not in alphabet".into())
                    .and_then(|idx| {
                        Char::try_from(idx).map_err(|_| "Index conversion failed".into())
                    })
            })
            .collect::<Result<Vec<Char>, Box<dyn std::error::Error>>>()?;
        Ok(Self::new(&alphabet, x))
    }

    /// Creates a new `SizedStr` from a string slice using the same alphabet
    /// as the current `SizedStr`. This maps the  two strings to the same string-space,
    /// so they can be manipulated together.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to convert.
    ///
    /// # Returns
    ///
    /// A new `SizedStr` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{SizedAlphabet, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']).unwrap());
    /// let chars = vec![1u8, 2, 3];
    /// let s1 = SizedStr::new(&alphabet, chars);
    /// let s2 = s1.translate_to_this_alphabet("abc").unwrap();
    /// assert_eq!(s2[0], 1);
    /// assert_eq!(s2[1], 2);
    /// assert_eq!(s2[2], 3);
    /// ```
    pub fn translate_to_this_alphabet(&self, s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = self.alphabet.clone();
        Self::from_str_with_alphabet(s, &alphabet)
    }

    /// Returns the length of the string.
    ///
    /// # Returns
    ///
    /// The length of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{SizedAlphabet, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(SizedAlphabet::new(&['a', 'b', 'c']).unwrap());
    /// let chars = vec![1u8, 2, 3];
    /// let s = SizedStr::new(&alphabet, chars);
    /// assert_eq!(s.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.char_vector.len()
    }
}

impl<Char: CharacterTrait> std::ops::Index<usize> for SizedStr<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.char_vector[index]
    }
}

impl<Char: CharacterTrait> std::ops::IndexMut<usize> for SizedStr<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.char_vector[index]
    }
}
