use crate::strict_border_array;
use crate::utils::{CharacterTrait, Str, StrMapper, StrMappers};

struct KMPSearch<Char: CharacterTrait> {
    /// The string we are searching in
    x: Str<Char>,
    /// The pattern we are searching for
    p: Str<Char>,
    /// The border array of the pattern
    ba: Vec<usize>,
    /// The current index in the string
    x_index: usize,
    /// The current index in the pattern
    p_index: usize,
}

impl<Char: CharacterTrait> KMPSearch<Char> {
    fn new(x: Str<Char>, p: Str<Char>, ba: Vec<usize>) -> KMPSearch<Char> {
        KMPSearch {
            x,
            p,
            ba,
            x_index: 0,
            p_index: 0,
        }
    }
}

impl<Char: CharacterTrait> Iterator for KMPSearch<Char> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let KMPSearch {
            x,
            p,
            ba,
            x_index: i,
            p_index: j,
            ..
        } = self;

        let n = x.len();
        let m = p.len();

        while *i < n {
            // Shift pattern until it matches the border
            while *j > 0 && &x[*i] != &p[*j] {
                *j = ba[*j - 1];
            }

            // Move one step forward (if we can)
            if &x[*i] == &p[*j] {
                *j += 1;
            }

            // Move to the next position (saving it for the next call to next)
            *i += 1;

            // Return if a match was found
            if *j == m {
                *j = ba[*j - 1];
                return Some(*i - m);
            }
        }
        None
    }
}

/// Returns an iterator over the starting indices of occurrences of the pattern
/// `p` in the text `x` using the Knuth-Morris-Pratt (KMP) string matching algorithm.
///
/// The KMP algorithm uses the border array to efficiently find occurrences of
/// the pattern `p` in the text `x`. It shifts the pattern according to the
/// border array to avoid unnecessary comparisons.
///
/// The algorithm runs in O(n + m) time, where `n` is the length of the text, and
/// `m` is the length of the pattern.
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
/// use stralg::kmp;
///
/// let text = "abracadabra";
/// let pattern = "abr";
/// let matches: Vec<usize> = kmp(text, pattern).collect();
/// assert_eq!(matches, vec![0, 7]);
/// ```
pub fn kmp(x: &str, p: &str) -> Box<dyn Iterator<Item = usize>> {
    if x.is_empty() || p.is_empty() {
        return Box::new(std::iter::empty());
    }
    if x.len() < p.len() {
        return Box::new(std::iter::empty());
    }

    let mapper = StrMappers::new_from_str(x).unwrap(); // We unwrap because we don't expect alphabet larger than u16
    match mapper {
        StrMappers::U8Mapper(mapper) => kmp_impl(x, p, mapper),
        StrMappers::U16Mapper(mapper) => kmp_impl(x, p, mapper),
    }
}

fn kmp_impl<Char>(x: &str, p: &str, mapper: StrMapper<Char>) -> Box<dyn Iterator<Item = usize>>
where
    Char: CharacterTrait,
{
    let x = mapper.map_str(x).unwrap(); // We built the string from x so this cannot fail...
    let p = match mapper.map_str(p) {
        Ok(p) => p,
        Err(_) => return Box::new(std::iter::empty()),
    };
    let ba = strict_border_array(&p);
    Box::new(KMPSearch::new(x, p, ba))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kmp() {
        let x = "abracadabra";
        let p = "abr";
        let result: Vec<usize> = kmp(x, p).collect();
        assert_eq!(result, vec![0, 7]);
    }
}
