use crate::utils::{Alphabet, SizedAlphabet, SizedStr};

struct NaiveSearch {
    x: SizedStr<u8>,
    p: Option<SizedStr<u8>>,
    i: usize,
}

impl Iterator for NaiveSearch {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let NaiveSearch { x, p, i } = self;
        let p = match p {
            Some(p) => p,
            None => return None,
        };

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
pub fn naive(x: &str, p: &str) -> impl Iterator<Item = usize> {
    // FIXME: pick size of alphabet based on input
    let x = SizedStr::<u8>::from_str(x).unwrap();
    let p = x.translate_to_this_alphabet(p).ok();
    let i = 0;
    NaiveSearch { x, p, i }
}
