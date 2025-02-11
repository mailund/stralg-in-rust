use std::collections::HashMap;

pub trait AlphabetChar: Eq + std::hash::Hash + TryFrom<usize> + Copy + Into<usize> {
    type Error: std::fmt::Debug;
}
impl AlphabetChar for u8 {
    type Error = std::num::TryFromIntError;
}
impl AlphabetChar for u16 {
    type Error = std::num::TryFromIntError;
}

/// An alphabet we can have strings over.
///
/// This is predominantly used for mapping UTF-8 str strings to vectors where we have
/// constant time access to the characters, without relying on a Vec<char> which would take
/// up four bytes per character.
pub struct SizedAlphabet<Char>
where
    Char: AlphabetChar,
{
    chars: Vec<char>,
    indices: HashMap<char, usize>,
    _phantom: std::marker::PhantomData<Char>,
}

impl<Char> SizedAlphabet<Char>
where
    Char: AlphabetChar,
{
    /// Creates a new `SizedAlphabet` from a slice of characters.
    ///
    /// # Arguments
    ///
    /// * `chars` - A slice of characters to include in the alphabet.
    ///
    /// # Returns
    ///
    /// A `SizedAlphabet` containing the given characters and their corresponding indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn new(chars: &[char]) -> SizedAlphabet<Char> {
        let len = chars.len();
        let mut indices = HashMap::with_capacity(len);
        for (i, &c) in chars.iter().enumerate() {
            indices.insert(c, i + 1); // The +1 is to leave room for the sentinel at zero
        }
        SizedAlphabet::<Char> {
            chars: chars.to_vec(),
            indices,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `SizedAlphabet` from a string.
    ///
    /// The alphabet will contain the characters in the string plus zero as a sentinel.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to include in the alphabet.
    ///
    /// # Returns
    ///
    /// A `SizedAlphabet` containing the characters in the string and their corresponding indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let s = "abc";
    /// let alphabet = SizedAlphabet::<u8>::from_str(s);
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn from_str(s: &str) -> SizedAlphabet<Char> {
        let chars: Vec<char> = s.chars().collect();
        SizedAlphabet::new(&chars)
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
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// let translated = alphabet.translate("abc").unwrap();
    /// assert_eq!(translated.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn translate(&self, s: &str) -> Result<Vec<Char>, Box<dyn std::error::Error>> {
        s.chars()
            .map(|c| {
                self.index(c)
                    .ok_or_else(|| "Character not in alphabet".into())
                    .and_then(|idx| {
                        Char::try_from(idx).map_err(|_| "Index conversion failed".into())
                    })
            })
            .collect()
    }

    /// Translates a vector of character indices back into a string.
    ///
    /// # Arguments
    ///
    /// * `vec` - A `Vec<Char>` containing the indices to translate.
    ///
    /// # Returns
    ///
    /// A `String` representing the characters corresponding to the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// let translated = alphabet.translate("abc").unwrap();
    /// let s = alphabet.as_string(&translated).unwrap();
    /// assert_eq!(s, "abc");
    /// ```
    pub fn as_string(&self, vec: &Vec<Char>) -> Result<String, Box<dyn std::error::Error>> {
        vec.iter()
            .map(|&c| {
                self.chars
                    .get(c.into() - 1) // -1 to compensate for zero sentinel
                    .ok_or_else(|| "Index out of bounds".into())
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|chars| chars.into_iter().collect())
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
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// assert!(alphabet.contains('a'));
    /// assert!(!alphabet.contains('d'));
    /// ```
    pub fn contains(&self, c: char) -> bool {
        self.indices.contains_key(&c)
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
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.index('d'), None);
    /// ```
    pub fn index(&self, c: char) -> Option<usize> {
        self.indices.get(&c).copied()
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
    /// use stralg::utils::SizedAlphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = SizedAlphabet::<u8>::new(&chars);
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.chars.len()
    }
}
