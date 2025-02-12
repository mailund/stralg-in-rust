use super::{Alphabet, AlphabetImpl, CharacterTrait};
use std::rc::Rc;

pub enum Str {
    U8(SizedStr<u8>),
    U16(SizedStr<u16>),
}

impl Str {
    fn new(alphabet: &Rc<Alphabet>, x: &str) -> Self {
        use Str::{U16, U8};
        match alphabet.as_ref() {
            Alphabet::U8(alphabet) => {
                let alphabet = alphabet.clone();
                let x = SizedStr::<u8>::from_str_with_alphabet(x, &alphabet).unwrap();
                U8(x)
            }
            Alphabet::U16(alphabet) => {
                let alphabet = alphabet.clone();
                let x = SizedStr::<u16>::from_str_with_alphabet(x, &alphabet).unwrap();
                U16(x)
            }
        }
    }

    pub fn from_str(x: &str) -> Self {
        let alphabet = Alphabet::from_str(x).unwrap();
        Self::new(&Rc::new(alphabet), x)
    }

    pub fn from_str_with_alphabet(x: &str, alphabet: &Rc<Alphabet>) -> Self {
        Self::new(alphabet, x)
    }

    pub fn translate_to_this_alphabet(&self, s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use Str::{U16, U8};
        let translated = match self {
            U8(x) => U8(x.translate_to_this_alphabet(s)?),
            U16(x) => U16(x.translate_to_this_alphabet(s)?),
        };
        Ok(translated)
    }

    pub fn len(&self) -> usize {
        match self {
            Str::U8(s) => s.len(),
            Str::U16(s) => s.len(),
        }
    }
}

/// A string type that uses a custom alphabet for character encoding.
pub struct SizedStr<Char: CharacterTrait> {
    pub alphabet: Rc<AlphabetImpl<Char>>,
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
    /// use stralg::utils::{AlphabetImpl, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(AlphabetImpl::new(&['a', 'b', 'c']).unwrap());
    /// let chars = vec![1u8, 2, 3];
    /// let s = SizedStr::new(&alphabet, chars);
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn new(alphabet: &Rc<AlphabetImpl<Char>>, x: Vec<Char>) -> Self {
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
        let alphabet: Rc<AlphabetImpl<Char>> = Rc::new(AlphabetImpl::from_str(s)?);
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
    /// use stralg::utils::{AlphabetImpl, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(AlphabetImpl::new(&['a', 'b', 'c']).unwrap());
    /// let s = SizedStr::<u8>::from_str_with_alphabet("abc", &alphabet).unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str_with_alphabet(
        s: &str,
        alphabet: &Rc<AlphabetImpl<Char>>,
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
    /// use stralg::utils::{AlphabetImpl, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(AlphabetImpl::new(&['a', 'b', 'c']).unwrap());
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
    /// use stralg::utils::{AlphabetImpl, SizedStr};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(AlphabetImpl::new(&['a', 'b', 'c']).unwrap());
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
