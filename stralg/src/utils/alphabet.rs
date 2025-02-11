use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;

/// Represents an alphabet with a set of characters and their corresponding indices.
pub enum Alphabet {
    U8(AlphabetImpl<u8>),
    U16(AlphabetImpl<u16>),
}

/// Represents a vector of character indices, which can be of type `u8` or `u16`.
#[derive(Clone, PartialEq, Eq)]
pub enum CharArray {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

impl Alphabet {
    /// Creates a new `Alphabet` from a slice of characters.
    ///
    /// # Arguments
    ///
    /// * `chars` - A slice of characters to include in the alphabet.
    ///
    /// # Returns
    ///
    /// An `Alphabet` containing the given characters and their corresponding indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(1));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn new(chars: &[char]) -> Result<Alphabet, Box<dyn std::error::Error>> {
        let len = chars.len();
        if len <= u8::MAX as usize {
            Ok(Alphabet::U8(AlphabetImpl::new(chars)?))
        } else if len <= u16::MAX as usize {
            Ok(Alphabet::U16(AlphabetImpl::new(chars)?))
        } else {
            Err(format!("Alphabet too large: {}", len).into())
        }
    }

    /// Creates a new `Alphabet` from a string.
    ///
    /// The alphabet will contain the characters in the string plus zero as a sentinel.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to include in the alphabet.
    ///
    /// # Returns
    ///
    /// An `Alphabet` containing the characters in the string and their corresponding indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let s = "abc";
    /// let alphabet = Alphabet::from_string(s).unwrap();
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(1));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn from_string(s: &str) -> Result<Alphabet, Box<dyn std::error::Error>> {
        let chars: Vec<char> = s.chars().collect();
        Alphabet::new(&chars)
    }

    /// Translates a string slice into a vector of the underlying type in the implementation.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to translate.
    ///
    /// # Returns
    ///
    /// A vector of the underlying type representing the indices of the characters in the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// let translated = alphabet.translate("abc").unwrap();
    /// assert_eq!(translated.to_vec(), vec![0, 1, 2]);
    /// ```
    pub fn translate(&self, s: &str) -> Result<CharArray, Box<dyn std::error::Error>> {
        match self {
            Alphabet::U8(impl_) => impl_.translate(s).map(CharArray::U8),
            Alphabet::U16(impl_) => impl_.translate(s).map(CharArray::U16),
        }
    }

    /// Translates a vector of character indices back into a string.
    ///
    /// # Arguments
    ///
    /// * `array` - A `CharArray` containing the indices to translate.
    ///
    /// # Returns
    ///
    /// A `String` representing the characters corresponding to the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::{Alphabet, CharArray};
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// let translated = alphabet.translate("abc").unwrap();
    /// let s = alphabet.as_string(&translated).unwrap();
    /// assert_eq!(s, "abc");
    /// ```
    pub fn as_string(&self, array: &CharArray) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Alphabet::U8(impl_) => impl_.as_string(array.to_u8().unwrap()),
            Alphabet::U16(impl_) => impl_.as_string(array.to_u16().unwrap()),
        }
    }
    /// Checks if the alphabet contains the given character.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to check for.
    ///
    /// # Returns
    ///
    /// `true` if the alphabet contains the character, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// assert!(alphabet.contains('a'));
    /// assert!(!alphabet.contains('d'));
    /// ```
    pub fn contains(&self, c: char) -> bool {
        match self {
            Alphabet::U8(impl_) => impl_.contains(c),
            Alphabet::U16(impl_) => impl_.contains(c),
        }
    }

    /// Returns the index of the given character in the alphabet.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to get the index for.
    ///
    /// # Returns
    ///
    /// `Some(usize)` containing the index of the character if it exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// assert_eq!(alphabet.index('b'), Some(1));
    /// assert_eq!(alphabet.index('d'), None);
    /// ```
    pub fn index(&self, c: char) -> Option<usize> {
        match self {
            Alphabet::U8(impl_) => impl_.index(c).map(|i| i as usize),
            Alphabet::U16(impl_) => impl_.index(c).map(|i| i as usize),
        }
    }

    /// Returns the number of characters in the alphabet.
    ///
    /// The size of the alphabet does not include the sentinel
    /// character zero.
    ///
    /// # Returns
    ///
    /// The number of characters in the alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars).unwrap();
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Alphabet::U8(impl_) => impl_.len(),
            Alphabet::U16(impl_) => impl_.len(),
        }
    }
}

pub struct AlphabetImpl<T> {
    chars: Vec<char>,
    indices: HashMap<char, T>,
}

impl<T> AlphabetImpl<T>
where
    T: Copy + TryFrom<usize> + Into<usize>,
{
    fn new(chars: &[char]) -> Result<AlphabetImpl<T>, T::Error> {
        let mut sorted_chars = chars.to_vec();
        sorted_chars.sort_unstable();
        let mut indices = HashMap::new();
        for (i, &c) in sorted_chars.iter().enumerate() {
            indices.insert(c, T::try_from(i)?);
        }
        Ok(AlphabetImpl {
            chars: sorted_chars,
            indices,
        })
    }

    fn contains(&self, c: char) -> bool {
        self.indices.contains_key(&c)
    }

    fn index(&self, c: char) -> Option<T> {
        self.indices.get(&c).copied()
    }

    fn len(&self) -> usize {
        self.chars.len()
    }

    fn translate(&self, s: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        s.chars()
            .map(|c| {
                self.index(c)
                    .ok_or_else(|| format!("Character '{}' not found in alphabet", c).into())
            })
            .collect()
    }

    fn as_string(&self, array: &[T]) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        for &index in array {
            let index: usize = index.into();
            if index < self.chars.len() {
                result.push(self.chars[index]);
            } else {
                return Err(format!("Index '{}' out of bounds", index).into());
            }
        }
        Ok(result)
    }
}

impl CharArray {
    pub fn from_string(
        s: &str,
        alphabet: &Alphabet,
    ) -> Result<CharArray, Box<dyn std::error::Error>> {
        alphabet.translate(s)
    }

    pub fn len(&self) -> usize {
        match self {
            CharArray::U8(vec) => vec.len(),
            CharArray::U16(vec) => vec.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<usize> {
        match self {
            CharArray::U8(vec) => vec.iter().map(|&x| x as usize).collect(),
            CharArray::U16(vec) => vec.iter().map(|&x| x as usize).collect(),
        }
    }

    pub fn to_u8(&self) -> Option<&[u8]> {
        match self {
            CharArray::U8(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn to_u16(&self) -> Option<&[u16]> {
        match self {
            CharArray::U16(vec) => Some(vec),
            _ => None,
        }
    }
}

impl std::ops::Index<usize> for CharArray {
    type Output = usize;

    fn index(&self, index: usize) -> Self::Output {
        match self {
            CharArray::U8(vec) => vec[index] as usize,
            CharArray::U16(vec) => vec[index] as usize,
        }
    }
}

/*
impl std::ops::IndexMut<usize> for CharArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            CharArray::U8(vec) => &mut vec[index],
            CharArray::U16(vec) => &mut vec[index],
        }
    }
}
*/

impl std::fmt::Debug for CharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharArray::U8(vec) => vec.fmt(f),
            CharArray::U16(vec) => vec.fmt(f),
        }
    }
}

impl std::fmt::Display for CharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharArray::U8(vec) => vec.fmt(f),
            CharArray::U16(vec) => vec.fmt(f),
        }
    }
}
