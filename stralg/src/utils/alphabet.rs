use std::collections::HashMap;

/// An alphabet we can have strings over.
///
/// This is predominantly used for mapping UTF-8 str strings to vectors where we have
/// constant time access to the characters, without relying on a Vec<char> which would take
/// up four bytes per character.
pub struct Alphabet {
    chars: Vec<char>,
    indices: HashMap<char, usize>,
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
    /// let alphabet = Alphabet::new(&chars);
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn new(chars: &[char]) -> Alphabet {
        let len = chars.len();
        let mut indices = HashMap::with_capacity(len);
        for (i, &c) in chars.iter().enumerate() {
            indices.insert(c, i + 1); // The +1 is to leave room for the sentinel at zero
        }
        Alphabet {
            chars: chars.to_vec(),
            indices,
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
    /// let alphabet = Alphabet::from_str(s);
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn from_str(s: &str) -> Alphabet {
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
    /// let alphabet = Alphabet::new(&chars);
    /// let translated = alphabet.translate::<u8>("abc").unwrap();
    /// assert_eq!(translated.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn translate<C>(&self, s: &str) -> Result<Vec<C>, Box<dyn std::error::Error>>
    where
        C: TryFrom<usize> + Copy,
        <C as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        s.chars()
            .map(|c| {
                self.index(c)
                    .ok_or_else(|| "Character not in alphabet".into())
                    .and_then(|idx| C::try_from(idx).map_err(|_| "Index conversion failed".into()))
            })
            .collect()
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
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars);
    /// let translated = alphabet.translate::<u8>("abc").unwrap();
    /// let s = alphabet.as_string(&translated).unwrap();
    /// assert_eq!(s, "abc");
    /// ```
    pub fn as_string<C>(&self, vec: &Vec<C>) -> Result<String, Box<dyn std::error::Error>>
    where
        C: Into<usize> + Copy,
    {
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
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars);
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
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars);
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
    /// use stralg::utils::Alphabet;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let alphabet = Alphabet::new(&chars);
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.chars.len()
    }
}
