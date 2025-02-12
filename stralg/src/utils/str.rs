use super::{Alphabet, CharSize, CharacterTrait};
use std::rc::Rc;

/// A string mapper that uses a custom alphabet for character encoding.
///
/// This enum is used to select the correct mapper, and the underlying character type (u8 or u16).
///
pub enum StrMappers {
    /// Mapping to u8 characters
    U8Mapper(StrMapper<u8>),
    /// Mapping to u16 characters
    U16Mapper(StrMapper<u16>),
}

impl StrMappers {
    /// Creates a new `StrMappers` instance from a given alphabet.
    ///
    /// When picking a mapper, the character type is selected based on the size of the alphabet.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    ///
    /// # Returns
    ///
    /// A new `StrMappers` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{Alphabet, StrMappers};
    /// use std::rc::Rc;
    ///
    /// let alphabet = Rc::new(Alphabet::from_str("abc"));
    /// let mapper = StrMappers::new(&alphabet);
    /// match mapper {
    ///    StrMappers::U8Mapper(_) => (),
    ///   _ => panic!("Expected StrMapper::U8"),
    /// }
    /// ```
    pub fn new(alphabet: &Rc<Alphabet>) -> Self {
        use CharSize::*;
        use StrMappers::*;
        match alphabet.char_size().unwrap() {
            U8 => U8Mapper(StrMapper::new(alphabet)),
            U16 => U16Mapper(StrMapper::new(alphabet)),
        }
    }

    /// Creates a new `StrMappers` instance from a string slice.
    ///
    /// This is a convenience method that creates an alphabet from the string slice.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to convert.
    ///
    /// # Returns
    ///
    /// A new `StrMappers` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::StrMappers;
    /// let mapper = StrMappers::new_from_str("abc").unwrap();
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = Rc::new(Alphabet::from_str(s));
        Ok(Self::new(&alphabet))
    }

    /// Creates a new `StrMappers` instance from an array of string slices.
    ///
    /// This is a convenience method that creates an alphabet from the array of string slices.
    ///
    /// # Arguments
    ///
    /// * `strings` - An array of string slices to convert.
    ///
    /// # Returns
    ///
    /// A new `StrMappers` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::StrMappers;
    /// let mapper = StrMappers::new_from_strs(&["abc", "def"]).unwrap();
    /// ```
    pub fn new_from_strs(strings: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let alphabet = Rc::new(Alphabet::from_strs(strings));
        Ok(Self::new(&alphabet))
    }
}

/// A string mapper that uses a custom alphabet for character encoding.
pub struct StrMapper<Char>
where
    Char: CharacterTrait,
{
    /// The alphabet used for character encoding.
    pub alphabet: Rc<Alphabet>,
    _phantom: std::marker::PhantomData<Char>,
}

impl<Char> StrMapper<Char>
where
    Char: CharacterTrait,
{
    /// Creates a new `StrMapper` from a given alphabet.
    ///
    /// # Arguments
    ///
    /// * `alphabet` - A reference-counted pointer to the alphabet.
    ///
    /// # Returns
    ///
    /// A new `StrMapper` instance.
    pub(self) fn new(alphabet: &Rc<Alphabet>) -> Self {
        Self {
            alphabet: alphabet.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Maps a string to a vector of characters using the alphabet.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to convert.
    ///
    /// # Returns
    ///
    /// A vector of characters.
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains characters not in the alphabet.
    pub fn map_str(&self, s: &str) -> Result<Str<Char>, Box<dyn std::error::Error>> {
        let char_vector = self.alphabet.map_str::<Char>(s)?;
        Ok(Str::new(char_vector, &self.alphabet))
    }
}

/// A string type that uses a custom alphabet for character encoding.
#[derive(Debug, PartialEq, Clone)]
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

    pub fn iter(&self) -> std::slice::Iter<Char> {
        self.char_vector.iter()
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
    fn test_str_mapper_large_alphabet() {
        let letters: Vec<char> = (0..=u8::MAX).map(|c| c as char).collect(); // Too many chars for u8 (with sentinel)
        let alphabet = Rc::new(Alphabet::new(&letters));
        let mapper = StrMappers::new(&alphabet);
        match mapper {
            StrMappers::U16Mapper(_) => (),
            _ => panic!("Expected StrMapper::U16"),
        }
    }

    /* Skipped for now because we unwrap in StrMappers::new instead of returning a Result
    #[test]
    fn test_way_too_large_alphabet() {
        let letters: Vec<char> = (0..=u16::MAX)
            .map(|c| std::char::from_u32(c).unwrap())
            .collect(); // Too many chars for u16
        let alphabet = Rc::new(Alphabet::new(&letters));
        let mapper = StrMappers::new(&alphabet);
        assert!(mapper.is_err());
    }
    */

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
        assert_eq!(result, Str::new(vec![1, 2, 3], &alphabet));
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

    #[test]
    fn test_str_mappers_new_from_str() {
        let mapper = StrMappers::new_from_str("abc").unwrap();
        match mapper {
            StrMappers::U8Mapper(_) => (),
            _ => panic!("Expected StrMapper::U8"),
        }
    }

    #[test]
    fn test_str_mappers_new_from_strs() {
        let mapper = StrMappers::new_from_strs(&["abc", "def"]).unwrap();
        match mapper {
            StrMappers::U8Mapper(_) => (),
            _ => panic!("Expected StrMapper::U8"),
        }
    }

    #[test]
    fn test_mapper_map_str() {
        let mapper = StrMappers::new_from_str("abc").unwrap();
        let mapper = match mapper {
            StrMappers::U8Mapper(mapper) => mapper,
            _ => panic!("Expected StrMapper::U8"),
        };
        let result = mapper.map_str("abc").unwrap();
        assert_eq!(result, Str::new(vec![1, 2, 3], &mapper.alphabet));
    }

    #[test]
    fn test_str_new() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let chars = vec![1u8, 2, 3];
        let s = Str::new(chars, &alphabet);
        assert_eq!(s[0], 1);
        assert_eq!(s[1], 2);
        assert_eq!(s[2], 3);
    }

    #[test]
    fn test_str_from_str() {
        let alphabet = Rc::new(Alphabet::from_str("abc"));
        let s = Str::<u8>::from_str("abc", &alphabet).unwrap();
        assert_eq!(s[0], 1);
        assert_eq!(s[1], 2);
        assert_eq!(s[2], 3);
    }

    #[test]
    fn test_from_str_alphabet_too_large() {
        let letters: Vec<char> = (0..=u8::MAX).map(|c| c as char).collect(); // Too many chars for u8 (with sentinel)
        let alphabet = Rc::new(Alphabet::new(&letters));
        let result = Str::<u8>::from_str("abc", &alphabet);
        assert!(result.is_err());
    }
}
