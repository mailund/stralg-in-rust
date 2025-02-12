use crate::utils::{CharacterTrait, Str, StrMapper, StrMappers};

struct NaiveSearch<Char: CharacterTrait> {
    x: Str<Char>,
    p: Str<Char>,
    i: usize,
}

impl<Char: CharacterTrait> Iterator for NaiveSearch<Char> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let NaiveSearch { x, p, i } = self;
        let n = x.len();
        let m = p.len();
        while *i <= n - m {
            let mut k = 0;
            while k < m && p[k] == x[*i + k] {
                k += 1;
            }
            *i += 1;
            if k == m {
                return Some(*i - 1);
            }
        }
        None
    }
}

fn naive_impl<Char>(x: &str, p: &str, mapper: StrMapper<Char>) -> Box<dyn Iterator<Item = usize>>
where
    Char: CharacterTrait,
{
    let x = mapper.map_str(x).unwrap();
    let p = match x.translate_to_this_alphabet(p) {
        Ok(p) => p,
        Err(_) => return Box::new(std::iter::empty()),
    };

    Box::new(NaiveSearch { x, p, i: 0 })
}

/// Returns an iterator over the starting indices of occurrences of the pattern
/// `p` in the text `x` using the naive string matching algorithm.
///
/// The naive algorithm checks for the pattern `p` at every position in the text
/// `x` from `0` to `n - m`, where `n` is the length of the text and `m` is the
/// length of the pattern. It compares the substring of `x` starting at each
/// position with `p` to find matches.
///
/// The algorithm runs in worst time O((n - m + 1) * m) and best time O(n - m + 1),
/// where `n` is the length of the text and `m` is the length of the pattern.
///
/// # Arguments
///
/// * `x` - The text in which to search for the pattern.
/// * `p` - The pattern to search for.
///
/// # Returns
///
/// An iterator over the starting indices of occurrences of the pattern `p` in
/// the text `x`.
///
/// # Examples
///
/// ```
/// use stralg::naive;
///
/// let text = "abracadabra";
/// let pattern = "abr";
/// let matches: Vec<usize> = naive(text, pattern).collect();
/// assert_eq!(matches, vec![0, 7]);
/// ```
///
/// ```
/// use stralg::naive;
///
/// let text = "aaaaa";
/// let pattern = "aa";
/// let matches: Vec<usize> = naive(text, pattern).collect();
/// assert_eq!(matches, vec![0, 1, 2, 3]);
/// ```
///
/// ```
/// use stralg::naive;
///
/// let text = "hello";
/// let pattern = "ll";
/// let matches: Vec<usize> = naive(text, pattern).collect();
/// assert_eq!(matches, vec![2]);
/// ```
pub fn naive(x: &str, p: &str) -> Box<dyn Iterator<Item = usize>> {
    if x.is_empty() || p.is_empty() {
        return Box::new(std::iter::empty());
    }

    let mapper = StrMappers::new_from_str(x).unwrap(); // We unwrap because we don't expect alphabet larger than u16
    match mapper {
        StrMappers::U8Mapper(mapper) => naive_impl(x, p, mapper),
        StrMappers::U16Mapper(mapper) => naive_impl(x, p, mapper),
    }
}
