use super::{Alphabet, CharSize, CharacterTrait};
use std::rc::Rc;

pub enum StrMappers {
    U8Mapper(StrMapper<u8>),
    U16Mapper(StrMapper<u16>),
}

impl StrMappers {
    pub fn new(alphabet: &Rc<Alphabet>) -> Self {
        use CharSize::*;
        use StrMappers::*;
        match alphabet.char_size().unwrap() {
            U8 => U8Mapper(StrMapper::new(alphabet)),
            U16 => U16Mapper(StrMapper::new(alphabet)),
        }
    }
}

pub struct StrMapper<Char>
where
    Char: CharacterTrait,
{
    alphabet: Rc<Alphabet>,
    _phantom: std::marker::PhantomData<Char>,
}

impl<Char> StrMapper<Char>
where
    Char: CharacterTrait,
{
    pub fn new(alphabet: &Rc<Alphabet>) -> Self {
        Self {
            alphabet: alphabet.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn map_str(&self, s: &str) -> Result<Vec<Char>, Box<dyn std::error::Error>> {
        self.alphabet.map_str(s)
    }
}

/// A string type that uses a custom alphabet for character encoding.
pub struct Str<Char: CharacterTrait> {
    char_vector: Vec<Char>,
    pub alphabet: Rc<Alphabet>,
}

impl<Char: CharacterTrait> Str<Char> {
    /// Creates a new `Str` from a given alphabet and a vector of characters.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    /// * `x` - A vector of characters.
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
    /// let s = Str::new(chars, &alphabet);
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn new(x: Vec<Char>, alphabet: &Rc<Alphabet>) -> Self {
        Self {
            char_vector: x,
            alphabet: alphabet.clone(),
        }
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
    /// let s = Str::<u8>::from_str("abc", &alphabet).unwrap();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[1], 2);
    /// assert_eq!(s[2], 3);
    /// ```
    pub fn from_str(s: &str, alphabet: &Rc<Alphabet>) -> Result<Self, Box<dyn std::error::Error>> {
        if alphabet.len() > Char::MAX {
            return Err("Alphabet too large for Char type".into());
        }

        let x = s
            .chars()
            .map(|c| alphabet.map_char(c))
            .collect::<Result<Vec<Char>, Box<dyn std::error::Error>>>()?;

        Ok(Self::new(x, &alphabet))
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
    /// use stralg::utils::{Alphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(Alphabet::new(&['a', 'b', 'c']));
    /// let chars = vec![1u8, 2, 3];
    /// let s1 = Str::new(chars, &alphabet);
    /// let s2 = s1.translate_to_this_alphabet("abc").unwrap();
    /// assert_eq!(s2[0], 1);
    /// assert_eq!(s2[1], 2);
    /// assert_eq!(s2[2], 3);
    /// ```
    pub fn translate_to_this_alphabet(&self, s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = self.alphabet.clone();
        Self::from_str(s, &alphabet)
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
    /// use stralg::utils::{Alphabet, Str};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(Alphabet::new(&['a', 'b', 'c']));
    /// let chars = vec![1u8, 2, 3];
    /// let s = Str::new(chars, &alphabet);
    /// assert_eq!(s.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.char_vector.len()
    }
}

impl<Char: CharacterTrait> std::ops::Index<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.char_vector[index]
    }
}

impl<Char: CharacterTrait> std::ops::IndexMut<usize> for Str<Char>
where
    Char: TryFrom<usize> + Copy,
    <Char as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.char_vector[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_mapper_new() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let mapper = StrMappers::new(&alphabet);
        match mapper {
            StrMappers::U8Mapper(_) => (),
            _ => panic!("Expected StrMapper::U8"),
        }
    }

    #[test]
    fn test_sized_str_mapper_new() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let mapper = StrMapper::<u8>::new(&alphabet);
        assert_eq!(mapper.alphabet.len(), 3);
    }

    #[test]
    fn test_sized_str_mapper_map_str() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let mapper = StrMapper::<u8>::new(&alphabet);
        let result = mapper.map_str("abc").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_sized_str_mapper_map_str_error() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let mapper = StrMapper::<u8>::new(&alphabet);
        let result = mapper.map_str("def");
        assert!(result.is_err());
    }

    #[test]
    fn test_sized_str_mapper_map_str_error_2() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let mapper = StrMapper::<u8>::new(&alphabet);
        let result = mapper.map_str("abcd");
        assert!(result.is_err());
    }
}
