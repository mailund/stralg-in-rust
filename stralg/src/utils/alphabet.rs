use super::CharacterTrait;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Alphabet {
    U8(Rc<SizedAlphabet<u8>>),
    U16(Rc<SizedAlphabet<u16>>),
}

impl Alphabet {
    pub fn new(chars: &[char]) -> Result<Self, Box<dyn std::error::Error>> {
        const SMALL: usize = u8::MAX as usize;
        const LARGE: usize = u16::MAX as usize;
        match chars.len() {
            0..SMALL => Ok(Alphabet::U8(Rc::new(SizedAlphabet::new(chars)?))),
            SMALL..LARGE => Ok(Alphabet::U16(Rc::new(SizedAlphabet::new(chars)?))),
            _ => Err("Alphabet too large".into()),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars: Vec<char> = s.chars().collect();
        Alphabet::new(&chars)
    }

    pub fn contains(&self, c: char) -> bool {
        match self {
            Alphabet::U8(alphabet) => alphabet.contains(c),
            Alphabet::U16(alphabet) => alphabet.contains(c),
        }
    }

    pub fn index(&self, c: char) -> Option<usize> {
        match self {
            Alphabet::U8(alphabet) => alphabet.index(c),
            Alphabet::U16(alphabet) => alphabet.index(c),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Alphabet::U8(alphabet) => alphabet.len(),
            Alphabet::U16(alphabet) => alphabet.len(),
        }
    }
}

/// An alphabet we can have strings over.
///
/// This is predominantly used for mapping UTF-8 str strings to vectors where we have
/// constant time access to the characters, without relying on a Vec<char> which would take
/// up four bytes per character.
pub struct SizedAlphabet<Char>
where
    Char: CharacterTrait,
{
    chars: Vec<char>,
    indices: HashMap<char, usize>,
    _phantom: std::marker::PhantomData<Char>,
}

impl<Char: CharacterTrait> SizedAlphabet<Char> {
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
    /// let alphabet = SizedAlphabet::<u8>::new(&chars).unwrap();
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn new(chars: &[char]) -> Result<SizedAlphabet<Char>, Box<dyn std::error::Error>> {
        let len = chars.len();
        if len > Char::MAX as usize {
            return Err("Alphabet too large for character type".into());
        };
        let mut indices = HashMap::with_capacity(len);
        for (i, &c) in chars.iter().enumerate() {
            indices.insert(c, i + 1); // The +1 is to leave room for the sentinel at zero
        }
        Ok(SizedAlphabet::<Char> {
            chars: chars.to_vec(),
            indices,
            _phantom: std::marker::PhantomData,
        })
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
    /// let alphabet = SizedAlphabet::<u8>::from_str(s).unwrap();
    /// assert!(alphabet.contains('a'));
    /// assert_eq!(alphabet.index('b'), Some(2));
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn from_str(s: &str) -> Result<SizedAlphabet<Char>, Box<dyn std::error::Error>> {
        let chars: Vec<char> = s.chars().collect();
        SizedAlphabet::new(&chars)
    }

    // /// Translates a string slice into a vector of the underlying type in the implementation.
    // ///
    // /// # Arguments
    // ///
    // /// * `s` - A string slice to translate.
    // ///
    // /// # Returns
    // ///
    // /// A vector of the underlying type representing the indices of the characters in the string.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use stralg::utils::SizedAlphabet;
    // ///
    // /// let chars = vec!['a', 'b', 'c'];
    // /// let alphabet = SizedAlphabet::<u8>::new(&chars).unwrap();
    // /// let translated = alphabet.translate("abc").unwrap();
    // /// assert_eq!(translated.to_vec(), vec![1, 2, 3]);
    // /// ```
    // pub fn translate(&self, s: &str) -> Result<Str<Char>, Box<dyn std::error::Error>> {
    //     let vec: Vec<Char> = s
    //         .chars()
    //         .map(|c| {
    //             self.index(c)
    //                 .ok_or_else(|| "Character not in alphabet".into())
    //                 .and_then(|idx| {
    //                     Char::try_from(idx).map_err(|_| "Index conversion failed".into())
    //                 })
    //         })
    //         .collect()?;
    //     Ok(Str::new(&Rc::new(self.clone()), vec))
    // }

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
    /// let alphabet = SizedAlphabet::<u8>::new(&chars).unwrap();
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
    /// let alphabet = SizedAlphabet::<u8>::new(&chars).unwrap();
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
    /// let alphabet = SizedAlphabet::<u8>::new(&chars).unwrap();
    /// assert_eq!(alphabet.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.chars.len()
    }
}
