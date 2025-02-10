use std::collections::HashMap;
use std::convert::TryFrom;

/// Represents an alphabet with a set of characters and their corresponding indices.
pub enum Alphabet {
    U8(AlphabetImpl<u8>),
    U16(AlphabetImpl<u16>),
    U32(AlphabetImpl<u32>),
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
            Ok(Alphabet::U32(AlphabetImpl::new(chars)?))
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
    /// let indices = alphabet.translate("abc").unwrap();
    /// assert_eq!(indices, vec![0, 1, 2]);
    /// ```
    pub fn translate(&self, s: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let mut result = Vec::with_capacity(s.len());
        for c in s.chars() {
            match self.index(c) {
                Some(index) => result.push(index),
                None => return Err(format!("Character '{}' not found in alphabet", c).into()),
            }
        }
        Ok(result)
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
            Alphabet::U32(impl_) => impl_.contains(c),
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
            Alphabet::U32(impl_) => impl_.index(c).map(|i| i as usize),
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
            Alphabet::U32(impl_) => impl_.len(),
        }
    }
}

pub struct AlphabetImpl<T> {
    chars: Vec<char>,
    indices: HashMap<char, T>,
}

impl<T> AlphabetImpl<T>
where
    T: Copy + TryFrom<usize>,
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
}
