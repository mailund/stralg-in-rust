use std::collections::HashMap;

use super::CharacterTrait;

/// An alphabet we can have strings over.
///
/// This is predominantly used for mapping UTF-8 str strings to vectors where we have
/// constant time access to the characters, without relying on a Vec<char> which would take
/// up four bytes per character.
pub struct Alphabet {
    /// A vector of characters storing the alphabet in a specific order.
    chars: Vec<char>,
    /// A hash map mapping each character to its index in the `chars` vector.
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
    /// A `Alphabet` containing the given characters and their corresponding indices.
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
    /// A `Alphabet` containing the characters in the string and their corresponding indices.
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

    /// Creates a new `Alphabet` from a slice of string slices.
    ///
    /// This function iterates over each string in the slice and collects every unique character,
    /// preserving the order in which they first appear. Duplicate characters are ignored, ensuring each
    /// character appears only once in the resulting alphabet.
    ///
    /// # Arguments
    ///
    /// * `strings` - A slice of string slices to include in the alphabet.
    ///
    /// # Returns
    ///
    /// A `Alphabet` containing the unique characters from the provided strings and their corresponding indices,
    /// where indexing starts at 1 (with zero reserved as a sentinel).
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let strings = ["hello", "world"];
    /// let alphabet = Alphabet::from_strs(&strings);
    ///
    /// // Check that the alphabet contains unique characters from "hello" and "world"
    /// assert!(alphabet.contains('h'));
    /// assert!(alphabet.contains('e'));
    /// assert!(alphabet.contains('l'));
    /// assert!(alphabet.contains('o'));
    /// assert!(alphabet.contains('w'));
    /// assert!(alphabet.contains('r'));
    /// assert!(alphabet.contains('d'));
    ///
    /// // The expected unique characters are: 'h', 'e', 'l', 'o', 'w', 'r', 'd'
    /// assert_eq!(alphabet.len(), 7);
    ///
    /// // Additional check for indexing (note: index values start at 1)
    /// assert_eq!(alphabet.index('h'), Some(1));
    /// ```
    pub fn from_strs(strings: &[&str]) -> Alphabet {
        let mut chars = Vec::new();
        for s in strings {
            for c in s.chars() {
                if !chars.contains(&c) {
                    chars.push(c);
                }
            }
        }
        Alphabet::new(&chars)
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

    /// Maps a Rust built-in character (char) to a character of another type (Char).
    ///
    /// # Arguments
    ///
    /// * `c` - The character to map.
    ///
    /// # Returns
    ///
    /// `Ok(Char)` if the character is in the alphabet and the conversion to `Char` is successful,
    /// `Err(Box<dyn std::error::Error>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let alphabet = Alphabet::from_str("abc");
    /// let result: u8 = alphabet.map_char('b').unwrap();
    /// assert_eq!(result, 2);
    /// ```
    pub fn map_char<Char>(&self, c: char) -> Result<Char, Box<dyn std::error::Error>>
    where
        Char: CharacterTrait,
    {
        if self.len() > Char::MAX {
            return Err("Alphabet too large for Char type".into());
        }

        let idx = match self.index(c) {
            None => return Err("Character not in alphabet".into()),
            Some(idx) => idx,
        };

        Char::try_from(idx).map_err(|_| "Index conversion failed".into())
    }

    /// Maps a Rust built-in string slice (str) to a vector of characters of another type (Char).
    ///
    /// This function maps each character in the string slice to a character of type `Char` using the
    /// `map_char` function. The resulting vector contains the mapped characters in the same order as
    /// they appear in the string slice, and essentially is another representation of the string, but
    /// unlike `str` it is not in UTF-8 encoding so we can index individual characters in constant time.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to map.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Char>)` if the conversion is successful, `Err(Box<dyn std::error::Error>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::utils::Alphabet;
    ///
    /// let alphabet = Alphabet::from_str("abc");
    /// let result: Vec<u8> = alphabet.map_str("abc").unwrap();
    /// assert_eq!(result, vec![1, 2, 3]);
    /// ```
    pub fn map_str<Char>(&self, s: &str) -> Result<Vec<Char>, Box<dyn std::error::Error>>
    where
        Char: CharacterTrait,
    {
        s.chars().map(|c| self.map_char(c)).collect()
    }
}
